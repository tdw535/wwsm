
import sqlalchemy as db
from sqlalchemy import create_engine, insert
from sqlalchemy import Table, Column, Integer, String, Double


db_name = 'score_board.db'
engine = db.create_engine('sqlite:///db//'+db_name,echo=True)
connection = engine.connect()

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

metadata = db.MetaData()
metadata.reflect(bind=engine)
scoreBoard = metadata.tables["score_board"]
print(scoreBoard.columns.keys())
# query = db.select([scoreBoard]) 
# ResultProxy = connection.execute(query)
# ResultSet = ResultProxy.fetchall()
# print(ResultProxy)

def InsertEntry(name, score):
  stmt = insert(DATA_TABLE).values(user_name=name,score=score)
  compiled = stmt.compile()
  with engine.connect() as conn:
    result = conn.execute(stmt)
    conn.commit()

def InspectEntries():
  query = db.select(
    scoreBoard.columns["id","user_name", "score"])
  with engine.connect() as conn:
    result = conn.execute(query).fetchall()
    print(result)

if __name__ =='__main__':
  # CreateDB()
  # InsertEntry("BeepBoop",1.0)
  # InsertEntry("BeepBoop",2.0)
  InspectEntries()
  print("Done")