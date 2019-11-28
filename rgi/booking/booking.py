#!/usr/bin/env python3

from sqlalchemy.ext.declarative import DeclarativeMeta
from sqlalchemy.ext.automap import automap_base
from sqlalchemy.orm import Session
from sqlalchemy import create_engine
#from mail import send_request, send_approval, send_denial
import sys
import json
import os
import re


Base = automap_base()
engine = create_engine("sqlite:///" + os.getenv("DATABASE_URL"))
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
    :return: Booking dictionary or {result: number}
    """
    results = session.query(Booking).filter(Booking.id == data["args"]["id"]).all()
    if len(results) == 1:
        q = json.dumps(results[0], cls=AlchemyEncoder)
        print("="*30, file=sys.stderr)
        print(q, file=sys.stderr)
        return q
    else:
        return json.dumps({"result": 1})


def list_(data):
    """
    List all the data
    :param data:
    :return: {results: array of result}
    """
    results = session.query(Booking).all()
    return json.dumps(results, cls=AlchemyEncoder)


def post(data):
    """
    Adds new data to db
    :param data: Booking dictionary by it's id
    :return: {result: number}
    """

    result = Booking()
    for key, value in data["data"].items():
        if value is None:
            continue
        print(key, value, file=sys.stderr)
        setattr(result, key, value)
    result.approved = False

    events = session.query(Booking).filter(Booking.approved == 1).\
                                    filter(Booking.begin_time <= result.end_time).\
                                    filter(Booking.end_time <= result.begin_time)
    for event in events:
        if event.rooms == 3:
            return json.dumps({"result": 2})
        elif event.rooms == result.rooms:
            return json.dumps({"result": 2})

    session.add(result)
    session.commit()
    return json.dumps({"result": 0, "id": result.id})


def filter(data):
    """
    filer data by room flag, begin_time and start_time
    :param data: roomflag(0/1/2/3), begin_time and start_time
    :return: {results: array of result}
    """

    reservations = session.query(Booking).filter(Booking.begin_time <= data["args"]["end_time"]).\
                                          filter(Booking.end_time <= data["args"]["begin_time"])
    if data["args"]["rooms"] != 3:
        reservations.filter(Booking.rooms == data["args"]["rooms"])

    results = reservations.all()
    return json.dumps({"results": results}, cls=AlchemyEncoder)



def patch(data):
    """
    Update data in the database
    :param data: Booking dictionary
    :return: {result: number}
    """

    results = session.query(Booking).filter(Booking.id == data["args"]["id"]).all()
    if len(results) == 1:
        result = results[0]
        for key, value in data["data"].items():
            setattr(result, key, value)
        session.add(result)
        session.commit()
        return json.dumps({"result": 0})
    else:
        return json.dumps({"result": 1})  # no result found by the id

def delete(data):
    """
    Deletes event by it's id
    :param data: {id}
    :return: {result: number}
    """

    results = session.query(Booking).filter(Booking.id == data["args"]["id"]).all()
    if len(results) == 1:
        session.delete(results[0])
        session.commit()
        return json.dumps({"result": 0})
    else:
        return json.dumps({"result": 1})  # no result found by the id

def approve(data):
    """
    Approvs event by it's id
    :param data: {id}
    :return: ?
    """

    results = session.query(Booking).filter(Booking.id == data["args"]["id"]).all()
    if len(results) == 1:
        result = results[0]
        setattr(result, "approved", 1)
        session.add(result)
        session.commit()
        return json.dumps({"result": 0})
    else:
        return json.dumps({"result": 1})  # no result found by the id


methods = {"list": list_, "get": get, "post": post, "patch": patch, "delete": delete, "approve" : approve}
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
