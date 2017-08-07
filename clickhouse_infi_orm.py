
from infi.clickhouse_orm import models, fields, engines
from infi.clickhouse_orm.database import Database


class Test(models.Model):

    id = fields.Int64Field()
    a = fields.StringField()
    b = fields.StringField()
    c = fields.StringField()
    d = fields.StringField()

    engine = engines.MergeTree('id', ('a', 'b', 'c', 'd'))

db_url = 'http://web1:8123'
db_name = 'csv_parser_db'
db_username = 'default'
db_password = 'default'
db = Database(db_name=db_name, db_url=db_url, username=db_username, password=db_password)
db.create_table(Test)

# Insert some data
db.insert([
    Test(id=i, a=str(i), b=str(i), c=str(i), d=str(i)) for i in xrange(10, 15)
])

# Read data
for row in db.select("SELECT * FROM {}.test".format(db_name), model_class=Test):
    print row.id, row.a, row.b, row.c, row.d
