import json
import os
import pickle
import sys

from alphadb import AlphaDB

if __name__ == "__main__":
    #### Id provided by JS script
    instance_id = sys.argv[2]

    db = AlphaDB()

    #### Get cached AlphaDB instance if exitsts
    instance_path = f"./.adbcache/{instance_id}"
    if os.path.isfile(instance_path):
        with open(instance_path, "rb") as f:
            conncreds = pickle.loads(f.read())
            db.connect(host=conncreds["host"], user=conncreds["user"], password=conncreds["password"], database=conncreds["database"], port=conncreds["port"])
    else:
        if not os.path.isdir("./.adbcache"):
            os.mkdir("./.adbcache")  ## To write cache to later

    #### Write AlphaDB instance to cache
    def write_cache(adb_instance):
        with open(instance_path, "wb") as f:
            pickle.dump(adb_instance, f)

    if sys.argv[1] == "check":
        print(json.dumps(db.check()))

    if sys.argv[1] == "connect":
        conncreds = {
            "host": sys.argv[3],
            "user": sys.argv[4],
            "password": sys.argv[5],
            "database": sys.argv[6],
            "port": int(sys.argv[7]),
        }

        print(json.dumps(db.connect(host=conncreds["host"], user=conncreds["user"], password=conncreds["password"], database=conncreds["database"], port=conncreds["port"])))
        write_cache(conncreds)

    if sys.argv[1] == "init":
        print(db.init())

    if sys.argv[1] == "status":
        print(json.dumps(db.status()))

    if sys.argv[1] == "update_queries":
        print(json.dumps(db.update_queries(version_source=json.loads(sys.argv[3]), update_to_version=None if sys.argv[4] == "undefined" else sys.argv[4], no_data=sys.argv[5] == "true")))

    if sys.argv[1] == "update":
        print(db.update(version_source=json.loads(sys.argv[3]), update_to_version=None if sys.argv[4] == "undefined" else sys.argv[4], no_data=sys.argv[5] == "true"))

    if sys.argv[1] == "vacate":
        print(db.vacate(confirm=sys.argv[3] == "true"))

    sys.stdout.flush()
