using Printf

# Shamelessly stolen from another article
# https://sydney4.medium.com/pells-equation-and-the-chakravala-method-d4587f141b4
function Chakravala(a::BigInt, b::BigInt, k::BigInt, d::BigInt) 
    (new_a, new_b, new_k) = NextTriple(a, b, k, MinM(a,b,k,d), d)

    if new_k != 1
        return Chakravala(new_a, new_b, new_k, d)
    else
        return new_a, new_b
    end
end

# Shamelessly stolen from another article
# https://sydney4.medium.com/pells-equation-and-the-chakravala-method-d4587f141b4
function MinM(a, b, k, d)
    # Find the starting m
    start_m = 0
    for i in 0:(abs(k)-1)
        if (a+(b*i)) % k == 0
            start_m = i
        end
    end

    # Define search space
    m_to_search = [start_m + abs(i*k) for i in 0:9]

    # Find the minimum
    least_m = start_m
    for m in m_to_search
        if abs(m*m - d) < abs(least_m*least_m - d)
            least_m = m
        end
    end

    return least_m
end

# Shamelessly stolen from another article
# https://sydney4.medium.com/pells-equation-and-the-chakravala-method-d4587f141b4
function NextTriple(a, b, k, m, d)
    new_a = BigInt(trunc((a*m + d*b) / abs(k)))
    new_b = BigInt(trunc((a + b*m) / abs(k)))
    new_k = BigInt(trunc((m*m - d) / k))

    return new_a, new_b, new_k
end

isSquare(n) = (sqrt(n) - trunc(sqrt(n))) < 1e-5

function main()
    d_list = [i for i in 2:1000 if !isSquare(i)]

    max_d = 0
    max_c = 0

    for d in d_list
        (c, _) = Chakravala(BigInt(1),BigInt(1),BigInt(1-d),BigInt(d))
        if c > max_c
            max_d = d
            max_c = c
        end
    end

    println(max_d)
end

main()