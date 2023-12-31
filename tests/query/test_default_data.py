import json

from alphadb.utils.query.default_data import create_default_data


def test_query():
    test_item = {
        "col1": "value1",
        "col2": 1,
        "col3": None,
        "col4": True,
        "col5": False,
        "col6": {"json": "test"},
    }

    json_string_value = json.dumps(test_item["col6"])

    assert create_default_data(table_name="test", item=test_item) == f"INSERT INTO `test` (col1,col2,col4,col5,col6) VALUES ('value1',1,true,false,'{json_string_value}');"
