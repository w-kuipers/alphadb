import sys

from alphadb import AlphaDB

if sys.argv[1] == "connect":
    db = AlphaDB()
    try:
        db.connect(host=sys.argv[2], user=sys.argv[3], password=sys.argv[4], database=sys.argv[5], port=int(sys.argv[6]))
    except Exception as e:
        print(e)

sys.stdout.flush()
