from sqlalchemy import Column, Integer, String, Boolean, DateTime
from sqlalchemy.ext.declarative import declarative_base
import sys


Base = declarative_base()

class Booking(Base):
    __tablename__ = "booking"

    id = Column(Integer, primary_key=True)
    name = Column(String, nullable=False)
    description = Column(String, nullable=False)
    rooms = Column(Integer, nullable=False)
    begin = Column(DateTime, nullable=False)
    end = Column(DateTime, nullable=False)
    layout = Column(Integer, nullable=True)
    approved = Column(Boolean, nullable=False)


def get(data):
    ...
def post(data):
    ...
def patch(data):
    ...
def delete(data):
    ...

methods = {"get": get, "post": post, "patch": patch, "delete": delete}


if __name__ == "__main__":
    data = json.load(sys.stdin)
    if len(sys.argv) < 2:
        methods["get"](data)
    else:
        methods[sys.argv[1]](data)
