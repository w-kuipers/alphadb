# AlphaDB

Database versioning toolset.

## Exceptions

#### NoConnection

The `NoConnection` exception is thrown when no mysql connection class is specified.

#### DBNotInitialized

The `DBNotInitialized` exception is thrown when the database is not yet initialized.
    Database.init() ## Will initialize the database and thus resolve the error

#### DBTemplateNoMatch

The `DBTemplateNoMatch` exception is thrown when de database was previously updated using another version source.
On initialization, a table `FMM_ID` is created. In this table the column `template` is used to save the version source template name. Make sure it matches.
