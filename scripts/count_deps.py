# https://qiita.com/nomaton/items/c7747e479786995618b5
# count_deps.py
import csv
import datetime
import re
import sys
import tarfile


def main():
    # relax csv field size limit (default value makes failure)
    csv.field_size_limit(sys.maxsize)

    if len(sys.argv) < 2:
        print("no arguments", file=sys.stderr)
        sys.exit(1)
    file_path = sys.argv[1]

    print("extracting tar ...")
    with tarfile.open(file_path, "r:*") as tar:
        for member in tar.getmembers():
            name = member.name.rstrip("/")
            if "/" not in name:
                date_label = name[:10]
            elif name.endswith("/data/crates.csv"):
                crates_csv = member
            elif name.endswith("/data/categories.csv"):
                categories_csv = member
            elif name.endswith("/data/crates_categories.csv"):
                crates_categories_csv = member
            elif name.endswith("/data/versions.csv"):
                versions_csv = member
            elif name.endswith("/data/dependencies.csv"):
                dependencies_csv = member

        print("loading crates ...")
        crates = collect_crates(tar.extractfile(crates_csv))
        print("loading categories ...")
        categories = collect_categories(tar.extractfile(categories_csv))
        bind_categories(crates, categories, tar.extractfile(crates_categories_csv))
        print("loading versions ...")
        bind_latest_version(crates, tar.extractfile(versions_csv))
        verid_to_crate = {}
        for crate in crates.values():
            verid_to_crate[crate.version.id] = crate
        print("loading dependencies ...")
        bind_users(crates, verid_to_crate, tar.extractfile(dependencies_csv))

    print("resolving indirect dependencies ...")
    progress_tab = [(len(crates) * n // 10 - 1, n * 10) for n in range(1, 11)]
    for n, crate in enumerate(crates.values()):
        all_users = crate.all_users
        rest = list(crate.dir_users)
        while rest:
            dir_user = rest.pop(0)
            all_users.add(dir_user)
            rest.extend(
                x for x in dir_user.dir_users if x not in all_users and x != crate
            )

        if n == progress_tab[0][0]:
            _, percent = progress_tab.pop(0)
            print("  {}%".format(percent))

    output_csv = "crates_" + date_label + ".csv"
    print("writing {} ...".format(output_csv))
    write_crates_csv(crates, output_csv)


def as_bool(s):
    return {"t": True, "f": False}[s]


def as_datetime(s):
    return datetime.datetime.strptime(s[:19], "%Y-%m-%d %H:%M:%S")


# re_semver ignores build metadata
re_semver = re.compile(r"(\d+\.\d+\.\d+)(\-([0-9A-Za-z\.\-]+))?")


def as_semver(s):
    m = re_semver.match(s)
    triple, pre = m.group(1, 3)
    # (major, minor, patch)
    triple = tuple(map(int, triple.split(".")))
    if pre:
        # map numeric to (0, int) and nonnumeric to (1, str) for rules:
        # - numeric < non-numeric
        # - numeric is compared as number
        # - non-numeric is compared alphabetically
        # e.g. alpha.2.11 -> ((1, 'alpha'), (0, 2), (0, 11))
        def pre_part_to_pair(part):
            return (0, int(part)) if part.isdigit() else (1, part)

        pre = tuple(map(pre_part_to_pair, pre.split(".")))

    return triple, pre


class Crate:
    __slots__ = [
        "id",
        "name",
        "categories",
        "desc",
        "created_at",
        "updated_at",
        "version",
        "dir_users",
        "dev_users",
        "all_users",
    ]


class Version:
    __slots__ = [
        "id",
        "num",
        "triple",
        "pre",
        "yanked",
    ]

    def preferred_to(self, other):
        if self.yanked != other.yanked:
            return not self.yanked
        elif bool(self.pre) != bool(other.pre):
            return not self.pre
        elif self.triple != other.triple:
            return self.triple > other.triple
        else:
            return self.pre and other.pre and self.pre > other.pre


def collect_crates(csv_file):
    crates = {}
    reader = csv.DictReader(map(lambda b: b.decode("utf-8"), csv_file))

    for row in reader:
        crate = Crate()
        crate.id = int(row["id"])
        crate.name = row["name"]
        crate.categories = []
        crate.desc = row["description"]
        crate.created_at = as_datetime(row["created_at"])
        crate.updated_at = as_datetime(row["updated_at"])
        crate.version = None
        crate.dir_users = set()
        crate.dev_users = set()
        crate.all_users = set()

        crates[crate.id] = crate

    return crates


def collect_categories(csv_file):
    categories = {}
    reader = csv.DictReader(map(lambda b: b.decode("utf-8"), csv_file))

    for row in reader:
        id_ = int(row["id"])
        category = row["category"]
        categories[id_] = category

    return categories


def bind_categories(crates, categories, csv_file):
    reader = csv.DictReader(map(lambda b: b.decode("utf-8"), csv_file))

    for row in reader:
        crate = crates[int(row["crate_id"])]
        category = categories[int(row["category_id"])]
        crate.categories.append(category)


def bind_latest_version(crates, csv_file):
    reader = csv.DictReader(map(lambda b: b.decode("utf-8"), csv_file))

    for row in reader:
        version = Version()
        version.id = int(row["id"])
        version.num = row["num"]
        version.triple, version.pre = as_semver(version.num)
        version.yanked = as_bool(row["yanked"])

        crate_id = int(row["crate_id"])
        crate = crates[crate_id]
        if crate.version is None or version.preferred_to(crate.version):
            crate.version = version


def bind_users(crates, verid_to_crate, csv_file):
    reader = csv.DictReader(map(lambda b: b.decode("utf-8"), csv_file))
    for row in reader:
        verid = int(row["version_id"])
        source_crate = verid_to_crate.get(verid, None)
        if not source_crate:
            continue
        target_crate = crates[int(row["crate_id"])]
        kind = row["kind"]
        # 0: dependencies
        # 1: build_dependencies
        # 2: dev_dependencies
        if kind == "0":
            target_crate.dir_users.add(source_crate)
        elif kind in ("1", "2"):
            target_crate.dev_users.add(source_crate)


def write_crates_csv(crates, csv_path):
    fieldnames = [
        "name",
        "version",
        "created_at",
        "updated_at",
        "categories",
        "description",
        "dir_users_count",
        "all_users_count",
        "dev_users_count",
    ]
    with open(csv_path, "w", encoding="utf-8") as csv_file:
        writer = csv.DictWriter(csv_file, fieldnames)
        writer.writeheader()

        for crate in crates.values():
            row = {
                "name": crate.name,
                "version": crate.version.num,
                "created_at": crate.created_at,
                "updated_at": crate.updated_at,
                "categories": " | ".join(sorted(crate.categories)),
                "description": crate.desc,
                "dir_users_count": len(crate.dir_users),
                "all_users_count": len(crate.all_users),
                "dev_users_count": len(crate.dev_users),
            }
            writer.writerow(row)


if __name__ == "__main__":
    main()
