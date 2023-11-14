base = {
    "name": "test",
    "version": []
}

version = {
    "_id": "0.0.1"
}

altertable = {
    "altertable": {
        "test": {

        }
    }
}


def wrap_partial_in_altertable(partial):
   
    vs = base
    at = altertable
    at["altertable"]["test"] = partial
    vs["version"].append(version | at)

    return vs

def wrap_version_list_in_base(partial: list):

    b = base
    b["version"] = partial
    return b
