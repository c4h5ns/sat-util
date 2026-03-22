import sys
from .sat_python import cnf, solvers

sys.modules[f"{__name__}.cnf"] = cnf
sys.modules[f"{__name__}.solvers"] = solvers
sys.modules[f"{__name__}.solvers.fixstars"] = solvers.fixstars
sys.modules[f"{__name__}.solvers.kissat"] = solvers.kissat
