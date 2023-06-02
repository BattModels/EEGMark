# Heavily Based on https://gist.github.com/ChrisRackauckas/cc6ac746e2dfd285c28e0584a2bfd320
using OrdinaryDiffEq
using LinearAlgebra
using SparseArrays
using BenchmarkTools
using LoopVectorization
using Printf
using Zygote
using SciMLSensitivity

const α₂ = 1.0
const α₃ = 1.0
const β₁ = 1.0
const β₂ = 1.0
const β₃ = 1.0
const r₁ = 1.0
const r₂ = 1.0
const D = 100.0
const γ₁ = 0.1
const γ₂ = 0.1
const γ₃ = 0.1
const N = 128
const X = reshape([i for i in 1:N for j in 1:N], N, N)
const Y = reshape([j for i in 1:N for j in 1:N], N, N)
α₁ = 1.0 .* (X .>= 4 * N / 5)

const Mx = Tridiagonal([1.0 for i in 1:N-1], [-2.0 for i in 1:N], [1.0 for i in 1:N-1])
const My = copy(Mx)
Mx[2, 1] = 2.0
Mx[end-1, end] = 2.0
My[1, 2] = 2.0
My[end, end-1] = 2.0

function f!(_du, _u, _α₁, t)
    u = reshape(_u, N, N, 3)
    du = reshape(_du, N, N, 3)
    A = @view u[:, :, 1]
    B = @view u[:, :, 2]
    C = @view u[:, :, 3]
    dA = @view du[:, :, 1]
    dB = @view du[:, :, 2]
    dC = @view du[:, :, 3]
    α₁ = reshape(_α₁, N, N)

    @inbounds for j in 2:N-1, i in 2:N-1
        dA[i, j] = D * (A[i-1, j] + A[i+1, j] + A[i, j+1] + A[i, j-1] - 4A[i, j]) +
                   α₁[i, j] - β₁ * A[i, j] - r₁ * A[i, j] * B[i, j] + r₂ * C[i, j]
        dB[i, j] = α₂ - β₂ * B[i, j] - r₁ * A[i, j] * B[i, j] + r₂ * C[i, j]
        dC[i, j] = α₃ - β₃ * C[i, j] + r₁ * A[i, j] * B[i, j] - r₂ * C[i, j]
    end

    @inbounds for j in 2:N-1
        i = 1
        dA[1, j] = D * (2A[i+1, j] + A[i, j+1] + A[i, j-1] - 4A[i, j]) +
                   α₁[i, j] - β₁ * A[i, j] - r₁ * A[i, j] * B[i, j] + r₂ * C[i, j]
        dB[i, j] = α₂ - β₂ * B[i, j] - r₁ * A[i, j] * B[i, j] + r₂ * C[i, j]
        dC[i, j] = α₃ - β₃ * C[i, j] + r₁ * A[i, j] * B[i, j] - r₂ * C[i, j]
    end
    @inbounds for j in 2:N-1
        i = N
        dA[end, j] = D * (2A[i-1, j] + A[i, j+1] + A[i, j-1] - 4A[i, j]) +
                     α₁[i, j] - β₁ * A[i, j] - r₁ * A[i, j] * B[i, j] + r₂ * C[i, j]
        dB[i, j] = α₂ - β₂ * B[i, j] - r₁ * A[i, j] * B[i, j] + r₂ * C[i, j]
        dC[i, j] = α₃ - β₃ * C[i, j] + r₁ * A[i, j] * B[i, j] - r₂ * C[i, j]
    end

    @inbounds for i in 2:N-1
        j = 1
        dA[i, j] = D * (A[i-1, j] + A[i+1, j] + 2A[i, j+1] - 4A[i, j]) +
                   α₁[i, j] - β₁ * A[i, j] - r₁ * A[i, j] * B[i, j] + r₂ * C[i, j]
        dB[i, j] = α₂ - β₂ * B[i, j] - r₁ * A[i, j] * B[i, j] + r₂ * C[i, j]
        dC[i, j] = α₃ - β₃ * C[i, j] + r₁ * A[i, j] * B[i, j] - r₂ * C[i, j]
    end
    @inbounds for i in 2:N-1
        j = N
        dA[i, end] = D * (A[i-1, j] + A[i+1, j] + 2A[i, j-1] - 4A[i, j]) +
                     α₁[i, j] - β₁ * A[i, j] - r₁ * A[i, j] * B[i, j] + r₂ * C[i, j]
        dB[i, j] = α₂ - β₂ * B[i, j] - r₁ * A[i, j] * B[i, j] + r₂ * C[i, j]
        dC[i, j] = α₃ - β₃ * C[i, j] + r₁ * A[i, j] * B[i, j] - r₂ * C[i, j]
    end

    @inbounds begin
        i = 1
        j = 1
        dA[i, j] = D * (2A[i+1, j] + 2A[i, j+1] - 4A[i, j]) +
                   α₁[i, j] - β₁ * A[i, j] - r₁ * A[i, j] * B[i, j] + r₂ * C[i, j]
        dB[i, j] = α₂ - β₂ * B[i, j] - r₁ * A[i, j] * B[i, j] + r₂ * C[i, j]
        dC[i, j] = α₃ - β₃ * C[i, j] + r₁ * A[i, j] * B[i, j] - r₂ * C[i, j]

        i = 1
        j = N
        dA[i, j] = D * (2A[i+1, j] + 2A[i, j-1] - 4A[i, j]) +
                   α₁[i, j] - β₁ * A[i, j] - r₁ * A[i, j] * B[i, j] + r₂ * C[i, j]
        dB[i, j] = α₂ - β₂ * B[i, j] - r₁ * A[i, j] * B[i, j] + r₂ * C[i, j]
        dC[i, j] = α₃ - β₃ * C[i, j] + r₁ * A[i, j] * B[i, j] - r₂ * C[i, j]

        i = N
        j = 1
        dA[i, j] = D * (2A[i-1, j] + 2A[i, j+1] - 4A[i, j]) +
                   α₁[i, j] - β₁ * A[i, j] - r₁ * A[i, j] * B[i, j] + r₂ * C[i, j]
        dB[i, j] = α₂ - β₂ * B[i, j] - r₁ * A[i, j] * B[i, j] + r₂ * C[i, j]
        dC[i, j] = α₃ - β₃ * C[i, j] + r₁ * A[i, j] * B[i, j] - r₂ * C[i, j]

        i = N
        j = N
        dA[i, j] = D * (2A[i-1, j] + 2A[i, j-1] - 4A[i, j]) +
                   α₁[i, j] - β₁ * A[i, j] - r₁ * A[i, j] * B[i, j] + r₂ * C[i, j]
        dB[i, j] = α₂ - β₂ * B[i, j] - r₁ * A[i, j] * B[i, j] + r₂ * C[i, j]
        dC[i, j] = α₃ - β₃ * C[i, j] + r₁ * A[i, j] * B[i, j] - r₂ * C[i, j]
    end
end

u0 = zeros(N, N, 3)
prob = ODEProblem(f!, u0, (0.0, 10.0), α₁)

const EIGEN_EST = Ref(0.0f0)
EIGEN_EST[] = maximum(abs, eigvals(Matrix(My)))
function fz(p)
    mean(solve(
        prob,
        ROCK4(eigen_est=(integ) -> integ.eigen_est = EIGEN_EST[]),
        u0=vec(prob.u0),
        p=p,
        saveat=0.5,
        reltol=1e-8,
        abstol=1e-8,
        sensealg=InterpolatingAdjoint(autojacvec=ReverseDiffVJP(true))
    ))
end

# Setup benchmark
t = @benchmark Zygote.gradient(fz, vec(α₁))

# Score: Gradients Per Second
@printf("score: %f\n", 1/minimum(t).time)
