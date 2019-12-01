#!/usr/bin/env python3

from sqlalchemy.ext.declarative import DeclarativeMeta
from sqlalchemy.ext.automap import automap_base
from sqlalchemy.orm import Session
from sqlalchemy import create_engine
from mail import send_request, send_approval, send_denial
import sys
import json
import os
import re


Base = automap_base()
engine = create_engine("sqlite:///" + os.getenv("DATABASE_URL"))
Base.prepare(engine, reflect=True)
Booking = Base.classes.booking
User = Base.classes.users
session = Session(engine)

approver = "xsicp01@gjk.cz"

class AlchemyEncoder(json.JSONEncoder):

    def default(self, obj):
        if isinstance(obj.__class__, DeclarativeMeta):
            # an SQLAlchemy class
            fields = {}
            for field in [x for x in dir(obj) if not x.startswith('_') and x != 'metadata']:
                if field == "classes" or field == "prepare":
                    continue
                data = obj.__getattribute__(field)
                try:
                    json.dumps(data) # this will fail on non-encodable values, like other classes
                    fields[field] = data
                except TypeError:
                    fields[field] = None
            # a json-encodable dict
            return fields

        return json.JSONEncoder.default(self, obj)


def approve(data):
    """
    Approvs event by it's id
    :param data: {id}
    :return: ?
    """

    results = session.query(Booking).filter(Booking.id == data["args"]["id"]).all()
    if len(results) == 1:
        result = results[0]
        events = session.query(Booking).filter(Booking.approved == 1).\
                                    filter(Booking.begin_time <= result.end_time).\
                                    filter(Booking.end_time >= result.begin_time)
        for event in events:
            if event.rooms == 3:
                return json.dumps({"result": 2})
            elif event.rooms == result.rooms:
                return json.dumps({"result": 2})

        #send_approval("xsicp01@gjk.cz", "xsicp01@gjk.cz", result.rooms, result.begin_time, result.end_time)
        result.approved = 1
        session.add(result)
        session.commit()
        return json.dumps({"result": 0})
    else:
        return json.dumps({"result": 1})  # no result found by the id


methods = {"patch": patch, "approve" : approve}
txt = sys.stdin.read()
txt = re.sub(",[ \t\r\n]+}", "}", txt)
txt = re.sub(",[ \t\r\n]+\]", "]", txt)
print(txt, file=sys.stderr)
data = json.loads(txt)
if len(sys.argv) < 2:
    sys.stdout.write(methods["get"](data))
else:
    sys.stdout.write(methods[sys.argv[1].lower()](data))
sys.stdout.flush()
