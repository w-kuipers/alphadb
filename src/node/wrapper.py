import sys
import pickle
import os
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
            print(conncreds)
            db.connect(host=conncreds["host"], user=conncreds["user"], password=conncreds["password"], database=conncreds["database"], port=conncreds["port"])
    else:
        if not os.path.isdir("./.adbcache"):
            os.mkdir("./.adbcache")  ## To write cache to later

    #### Write AlphaDB instance to cache
    def write_cache(adb_instance):
        print("Starting writing cache")
        with open(instance_path, "wb") as f:
            pickle.dump(adb_instance, f)

    if sys.argv[1] == "connect":
        conncreds = {
            "host": sys.argv[3],
            "user": sys.argv[4],
            "password": sys.argv[5],
            "database": sys.argv[6],
            "port": sys.argv[7],
        }

        db.connect(host=sys.argv[3], user=sys.argv[4], password=sys.argv[5], database=sys.argv[6], port=int(sys.argv[7]))
        write_cache(conncreds)
        print("connect function ends hrer")

    if sys.argv[1] == "init":
        print("initializing")
        print(db.init())

    if sys.argv[1] == "status":
        print(db.status())

    sys.stdout.flush()
