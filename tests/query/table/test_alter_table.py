from src.alphadb.utils.query.table import alter_table


def test_drop_column():
    test_data = {"dropcolumn": ["col1", "col2", "col3"]}

    assert alter_table(table_data=test_data, table_name="test", version="0.0.1") == " ALTER TABLE `test` DROP COLUMN `col1`, DROP COLUMN `col2`, DROP COLUMN `col3`;"
