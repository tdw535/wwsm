
import sqlalchemy as db
from sqlalchemy import create_engine, insert
from sqlalchemy import Table, Column, Integer, String, Double

import json




class ScoreBoard:
  def __init__(self):
    self.db_name = 'score_board.db'
    self.engine = db.create_engine('sqlite:///db//'+self.db_name,echo=True)
    self.connection = self.engine.connect()

    self.metadata = db.MetaData()
    self.metadata.reflect(bind=self.engine)
    self.scoreBoard = self.metadata.tables["score_board"]


  def InsertEntry(self, name, score):
    stmt = insert(scoreBoard).values(user_name=name,score=score)
    compiled = stmt.compile()
    with self.engine.connect() as conn:
      result = conn.execute(stmt)
      conn.commit()

  def GetAllEntries(self):
    query = db.select(
      self.scoreBoard.columns["id","user_name", "score"])
    with self.engine.connect() as conn:
      result = conn.execute(query).fetchall()
      return result
  def GetAllEntriesAsJson(self):
    results = self.GetAllEntries()
    results = [tuple(row) for row in results]
    jsonString = json.dumps(results)
    return jsonString

if __name__ =='__main__':
  score_board = ScoreBoard()
  entries = score_board.GetAllEntries()
  print(entries)
