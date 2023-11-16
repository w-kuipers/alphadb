import pytest
from src.alphadb.utils.common import convert_version_number

def test_version_number_converts_to_int():
    assert convert_version_number("1.0.201") == 10201
    assert convert_version_number("0.5.0") == 50

    with pytest.raises(ValueError):
        convert_version_number("11a")
