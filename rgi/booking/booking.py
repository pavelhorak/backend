#!/usr/bin/env python3

from sqlalchemy.ext.declarative import DeclarativeMeta
from sqlalchemy.ext.automap import automap_base
from sqlalchemy.orm import Session
from sqlalchemy import create_engine
import sys
import json
import os
import re


Base = automap_base()
engine = create_engine(os.getenv("DATABASE_URL"))
Base.prepare(engine, reflect=True)
Booking = Base.classes.booking
session = Session(engine)


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


def get(data):
    """
    Get data from the database
    :param data: {id}
    :return: Booking dictionary or {error: (True/False)}
    """
    results = session.query(Booking).filter(Booking.id == data["args"]["id"]).all()
    if len(results) == 1:
        return json.dumps(results[0], cls=AlchemyEncoder)
    else:
        return json.dumps({"error": "Dawid Kubis to rozbil"})


def list_(data):
    results = session.query(Booking).all()
    return json.dumps({"results": results}, cls=AlchemyEncoder)


def post(data):
    """
    Adds new data to db
    :param data: Booking dictionary by it's id
    :return: {success: (True/"error message")}
    """

    result = Booking()
    for key, value in data["data"].items():
        setattr(result, key, value)
    result.approved = False

    events = session.query(Booking).filter(Booking.begin_time <= result.end_time).\
                                    filter(Booking.end_time <= result.start_time)
    for event in events:
        if event.rooms == 3:
            return json.dumps({"error": "Room is already used"})
        elif event.rooms == result.rooms:
            return json.dumps({"error": "Room is already used"})

    session.add(result)
    session.commit()
    return json.dumps({"success": True, "id": result.id})

def patch(data):
    """
    Update data in the database
    :param data: Booking dictionary
    :return: {success: (True/"error message")}
    """

    results = session.query(Booking).filter(Booking.id == data["args"]["id"]).all()
    if len(results) == 1:
        result = results[0]
        for key, value in data["data"].items():
            setattr(result, key, value)
        session.add(result)
        session.commit()
        return json.dumps({"success": True})
    else:
        return json.dumps({"error": "blame David Kubis for this one"})

def delete(data):
    """
    Deletes event by it's id
    :param data: {id}
    :return: {success: (True/False)}
    """

    results = session.query(Booking).filter(Booking.id == data["args"]["id"]).all()
    if len(results) == 1:
        session.delete(results[0])
        return json.dumps({"success": True})
    else:
        return json.dumps({"error": "Delete failed, bitches"})


methods = {"list": list_, "get": get, "post": post, "patch": patch, "delete": delete}
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
