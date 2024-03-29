import setuptools

with open("README.md", "r", encoding="utf-8") as fh:
    long_description = fh.read()

setuptools.setup(
    name="alphadb",

    #### ! Do not change the version number manually, this will be handled by the build script
    version="indev",

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
    python_requires=">=3.6",
    install_requires=["mysql-connector-python==8.2.0"],
)
