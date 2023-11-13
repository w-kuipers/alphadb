import setuptools

with open("README.md", "r", encoding="utf-8") as fh:
    long_description = fh.read()

setuptools.setup(
    name="alphadb",
    version="1.0.0a0",
    author="Wibo Kuipers",
    author_email="wibokuip@gmail.com",
    description="SQL Database versioning toolset",
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
    packages=["alphadb", "alphadb.utils", "alphadb.utils.query", "alphadb.utils.concatenate"],
    python_requires=">=3.6",
)
