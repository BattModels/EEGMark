using Pkg
using Printf
Pkg.instantiate()
stats = @timed Pkg.precompile()
@printf("score: %f\n", 3600 / stats.time) # Precompiles Per Hour
