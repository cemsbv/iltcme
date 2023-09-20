# Source: https://github.com/ghorvath78/iltcme

import cmath
import math
import json
import numpy as numpy

ltFuns = {
    "exponential": 
        lambda s: 1/(1+s),
    "sine": 
        lambda s: 1/(1+s**2),
    "heavyside": 
        lambda s: cmath.exp(-s)/s,
    "expheavyside": 
        lambda s: cmath.exp(-s)/(1+s),
    "squarewave": 
        lambda s: (1.0/s)*(1.0/(1.0+cmath.exp(s))),
    "staircase": 
        lambda s: (1.0/s)*(1.0/(cmath.exp(s)-1.0)),
}

def ilt(fun, T, maxFnEvals, iltcmeJson):
    assert(fun in ltFuns)
    ltFun = ltFuns[fun]

    if "cmeParams" not in globals():
        globals()["cmeParams"] = json.loads(iltcmeJson)
    # find the most steep CME satisfying maxFnEvals
    params = cmeParams[0]
    for p in cmeParams:
        if p["cv2"] < params["cv2"] and p["n"]+1 <= maxFnEvals:
            params = p
    eta = numpy.concatenate(([params["c"]], numpy.array(params["a"]) + 1j*numpy.array(params["b"])))*params["mu1"]
    beta = numpy.concatenate(([1], 1 + 1j*numpy.arange(1,params["n"]+1)*params["omega"]))*params["mu1"]

    res = [];
    for x in T:
        res.append(eta.dot([ltFun(b/x) for b in beta]).real/x)
    return res
