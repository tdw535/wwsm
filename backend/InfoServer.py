from flask import Flask
from flask_cors import CORS

import ScoreHandler as ScoreHandler

app = Flask(__name__)
cors = CORS(app)



class InfoServer:
  def __init__(self):
    self.score_board = ScoreHandler.ScoreBoard()
  def getScores(self):
    entries = self.score_board.GetAllEntriesAsJson()
    print(entries)
    return entries

infoServer = InfoServer()

@app.route("/")
def get_scoreboard():
    return infoServer.getScores()


@app.after_request
def after_request(response):
  response.headers.add('Access-Control-Allow-Origin', '*')
  response.headers.add('Access-Control-Allow-Headers', 'Content-Type,Authorization')
  response.headers.add('Access-Control-Allow-Methods', 'GET,PUT,POST,DELETE,OPTIONS')
  return response


if __name__ =='__main__':
  app.run(host="localhost",port=5057,debug=True)