
import sqlalchemy as db
from sqlalchemy import create_engine, insert
from sqlalchemy import Table, Column, Integer, String, Double


db_name = 'score_board.db'
engine = db.create_engine('sqlite:///db//'+db_name,echo=True)


# metadata_obj = MetaData()

# data_table = Table(
#   "score_board",
#   metadata_obj,
#   Column("id",Integer,primary_key=True),
#   Column("user_name", String(30)),
#   Column("score", Double),
# )


# def CreateDB():

#   metadata_obj.create_all(engine)

meta_data = db.MetaData(engine)
DATA_TABLE = meta_data.tables['score_board']


def InsertEntry(name, score):
  stmt = insert(DATA_TABLE).values(user_name=name,score=score)
  compiled = stmt.compile()
  with engine.connect() as conn:
    result = conn.execute(stmt)
    conn.commit()

def InspectEntries():
  query = db.select([
    DATA_TABLE.c.id,
    DATA_TABLE.c.user_name,
    DATA_TABLE.c.score,
  ])
  result = engine.execute(query).fetchall()
  print(result)

if __name__ =='__main__':
  # CreateDB()
  # InsertEntry("BeepBoop",1.0)
  # InsertEntry("BeepBoop",2.0)
  InspectEntries()
