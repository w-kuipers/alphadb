import setuptools

with open("README.md", "r", encoding="utf-8") as fh:
    long_description = fh.read()

version = "1.0.0b1"

def get_version():
    print(version)
    return version

setuptools.setup(
    name="alphadb",
    version=version,
    author="Wibo Kuipers",
    author_email="wibokuip@gmail.com",
    description="MySQL Database versioning toolset",
    long_description=long_description,
    long_description_content_type="text/markdown",
    url="https://github.com/w-kuipers/alphadb",
    project_urls={
        "Bug Tracker": "https://github.com/w-kuipers/alphadb/issues",
    },
    classifiers=[
        "Programming Language :: Python :: 3",
        "Operating System :: OS Independent",
    ],
    package_dir={"": "src"},
    packages=["alphadb", "alphadb.utils", "alphadb.utils.query", "alphadb.utils.query.column", "alphadb.utils.query.table", "alphadb.verification", "alphadb.utils.concatenate"],
    python_requires=">=3.6",
    install_requires=["mysql-connector-python==8.2.0"],
)
