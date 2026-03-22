import asyncio
from dotenv import load_dotenv
import os
from sat_python.cnf import Cnf
from sat_python.solvers import fixstars, kissat


load_dotenv(verbose=True)

FIXSTARS_ACCESS_TOKEN = os.environ.get("FIXSTARS_AMPLIFY_AE_ACCESS_TOKEN")
assert FIXSTARS_ACCESS_TOKEN is not None


async def main():
    cnf = Cnf(3, 3, 3, 1, "chacha8", True)
    print(cnf)
    print(cnf.to_dimacs_string())

    fixstars_context = fixstars.FixstarsSolverContext(
        access_token=FIXSTARS_ACCESS_TOKEN,
    )
    fixstars_solver = fixstars_context.create_solver(cnf)
    result = await fixstars_solver.solve()
    print(result.to_csv_record())

    kissat_context = kissat.KissatSolverContext(
        timeout=10,
    )
    kissat_solver = kissat_context.create_solver(cnf)
    result = await kissat_solver.solve()
    print(result.to_csv_record())


if __name__ == "__main__":
    asyncio.run(main())
