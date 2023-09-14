from flask import Flask
from flask_cors import CORS

app = Flask(__name__)
CORS(app)

@app.route("/api/score_board")
def hello_world():
    return "Hello, World!"

if __name__ =='__main__':
  app.run(host="localhost",port=5054,debug=True)