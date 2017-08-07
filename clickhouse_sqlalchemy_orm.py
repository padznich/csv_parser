
from sqlalchemy import create_engine, Column, Table, MetaData, literal
from clickhouse_sqlalchemy import types, engines
from clickhouse_sqlalchemy import make_session
from clickhouse_sqlalchemy.declarative import get_declarative_base


uri = 'clickhouse://default:default@web1:8123/csv_parser_db'
engine = create_engine(uri)
session = make_session(engine)
metadata = MetaData(bind=engine)

Base = get_declarative_base(metadata=metadata)


class Test(Base):
    id = Column(types.Int32, primary_key=True)
    a = Column(types.String)
    b = Column(types.String)
    c = Column(types.String)
    d = Column(types.String)

    __table_args__ = (
        engines.Memory(),
    )


test_table = Test.__table__

data = [{'id': i, 'a': str(i), 'b': str(i + 3), 'c': str(i + 3), 'd': str(i + 3)} for i in xrange(10)]

session.execute(test_table.insert(), data)
