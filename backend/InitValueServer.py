from flask import Flask
from flask_cors import CORS
import numpy as np

import json

app = Flask(__name__)
cors = CORS(app)



class InitValueServer:
  def __init__(self):
    self.file_path = r"/home/dev/Projects/wwsm/assets/initial_values_test.csv"
    self.init_val = np.genfromtxt(self.file_path, delimiter=",", usemask=True)
  def get_init_val(self):
    return self.init_val

initValServer = InitValueServer()
print(initValServer.get_init_val())

@app.route("/a")
def get_init_val():
    init_val = initValServer.get_init_val().tolist()
    jsonString = json.dumps(init_val)
    return jsonString


@app.after_request
def after_request(response):
  response.headers.add('Access-Control-Allow-Origin', '*')
  response.headers.add('Access-Control-Allow-Headers', 'Content-Type,Authorization')
  response.headers.add('Access-Control-Allow-Methods', 'GET,PUT,POST,DELETE,OPTIONS')
  return response


if __name__ =='__main__':
  app.run(host="localhost",port=5057,debug=True)