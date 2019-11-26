from sqlalchemy.ext.automap import automap_base
from sqlalchemy.orm import Session
from sqlalchemy import create_engine
import sys
import json
import os


Base = automap_base()
engine = create_engine(os.getenv("DATABASE_URL"))
Base.prepare(engine, reflect=True)
Booking = Base.classes.booking
session = Session(engine)


def get(data):
    """
    Get data from the database
    :param data: {id}
    :return: Booking dictionary
    """
    results = session.query(Booking).filter(Booking.id == data.id).all()
    if len(results) == 1:
        for result

    else:
        return {"error": True}


def post(data):
    """
    Adds new data to db
    :param data: Booking dictionary
    :return: success (True/False)
    """

def patch(data):
    """
    Update data in the database
    :param data: {id}
    :return: success (True/False)
    """


def delete(data):
    """
    Deletes event by it's id
    :param data: {id}
    :return: success (True/False)
    """


methods = {"get": get, "post": post, "patch": patch, "delete": delete}
data = json.load(sys.stdin)
if len(sys.argv) < 2:
    methods["get"](data)
else:
    methods[sys.argv[1]](data)
