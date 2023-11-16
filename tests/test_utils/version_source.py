
version = {
    "_id": "0.0.1"
}


def wrap_partial_in_altertable(partial):

    vs = {
        "name": "test",
        "version": []
    }
    at = {
        "altertable": {}
    }
    at["altertable"]["table"] = partial
    vs["version"].append(version | at)

    return vs

def wrap_partial_in_createtable(partial):

    vs = {
        "name": "test",
        "version": []
    }
    ct = {
        "createtable": {}
    }
    ct["createtable"]["table"] = partial
    vs["version"].append(version | ct)

    return vs

def wrap_version_list_in_base(partial: list):

    b =  {
        "name": "test",
        "version": []
    }
    b["version"] = partial
    return b
