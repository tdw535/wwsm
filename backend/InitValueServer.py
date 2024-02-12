from flask import Flask
from flask_cors import CORS
import numpy as np

import json

app = Flask(__name__)
cors = CORS(app)



class InitValueServer:
  def __init__(self):
    # self.file_path = r"/home/dev/Projects/wwsm/assets/initial_values_test.csv"
    self.file_path = r"/home/dev/Projects/wwsm/assets/sin1.csv"

    self.init_val = None
  def get_init_val(self):
    return np.genfromtxt(self.file_path, delimiter=",", usemask=True)


initValServer = InitValueServer()

@app.route("/a")
def get_init_val():
    init_val = initValServer.get_init_val()
    n_row = len(init_val)
    n_col = len(init_val[0])
    init_val_oneD = (init_val.ravel().tolist())

    init_val_as_dict = {
      "vals": init_val_oneD,
      "row": n_row,
      "col": n_col
    }

    jsonString = json.dumps(init_val_as_dict)
    return jsonString


@app.after_request
def after_request(response):
  response.headers.add('Access-Control-Allow-Origin', '*')
  response.headers.add('Access-Control-Allow-Headers', 'Content-Type,Authorization')
  response.headers.add('Access-Control-Allow-Methods', 'GET,PUT,POST,DELETE,OPTIONS')
  return response


if __name__ =='__main__':
  app.run(host="localhost",port=5057,debug=True)