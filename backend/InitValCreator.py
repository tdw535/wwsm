import numpy as np


wx = 2*np.pi
wy = 2*np.pi

nx = 20
ny = 40

dx = wx/nx
dy = wy/ny

x = np.arange(0,nx)*dx
y = np.arange(0,ny)*dy

xg, yg = np.meshgrid(x, y, indexing='ij')

Z = np.sin(xg)*np.sin(yg)
print(x)
print(y)
print(xg)
print(len(Z))

np.savetxt("../assets/sin1.csv", Z, delimiter=",")
np.savetxt("../assets/xgrid.csv",xg,delimiter=",")
np.savetxt("../assets/ygrid.csv",yg,delimiter=",")