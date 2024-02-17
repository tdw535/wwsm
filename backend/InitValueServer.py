from flask import Flask
from flask_cors import CORS
import numpy as np

import json

app = Flask(__name__)
cors = CORS(app)



class InitValueServer:
  def __init__(self):
    # self.file_path = r"/home/dev/Projects/wwsm/assets/initial_values_test.csv"
    self.base_path = r"/home/dev/Projects/wwsm/assets/"
    self.val_path = self.base_path + r"sin1.csv"
    self.xgrid_path = self.base_path + r"xgrid.csv"
    self.ygrid_path = self.base_path + r"ygrid.csv"

    self.init_val = None
  def get_values(self, which):
    path_to_use = self.val_path
    if (which == 'x'):
      path_to_use = self.xgrid_path
    if (which == 'y'):
      path_to_use = self.ygrid_path
    return np.genfromtxt(path_to_use, delimiter=",", usemask=True)

  def get_values_as_json(self, which):
    value = initValServer.get_values(which)
    n_row = len(value)
    n_col = len(value[0])
    value_oneD = (value.ravel().tolist())

    value_as_dict = {
      "vals": value_oneD,
      "row": n_row,
      "col": n_col
    }

    jsonString = json.dumps(value_as_dict)
    return jsonString
  
  def get_x_y_z_as_json(self):
    xval = initValServer.get_values('x')
    yval = initValServer.get_values('y')
    zval = initValServer.get_values('z')
    n_row = len(xval)
    n_col = len(xval[0])
    xval_oneD = (xval.ravel().tolist())
    yval_oneD = (yval.ravel().tolist())
    zval_oneD = (zval.ravel().tolist())

    value_as_dict = {
      "xg": xval_oneD,
      "yg": yval_oneD,
      "zg": zval_oneD,
      "row": n_row,
      "col": n_col
    }

    jsonString = json.dumps(value_as_dict)
    return jsonString     


initValServer = InitValueServer()

@app.route("/z")
def get_z():
    return initValServer.get_values_as_json('z')

@app.route("/x")
def get_x():
    return initValServer.get_values_as_json('x')

@app.route("/y")
def get_y():
    return initValServer.get_values_as_json('y')

@app.route("/xyz")
def get_xyz():
    return initValServer.get_x_y_z_as_json()


# @app.after_request
# def after_request(response):
#   response.headers.add('Access-Control-Allow-Origin', '*')
#   response.headers.add('Access-Control-Allow-Headers', 'Content-Type,Authorization')
#   response.headers.add('Access-Control-Allow-Methods', 'GET,PUT,POST,DELETE,OPTIONS')
#   return response


if __name__ =='__main__':
  app.run(host="localhost",port=5057,debug=True)