function divisors(n: number): number[] {
    for (let div = Math.sqrt(n) | 0; div > 1; div--)
        if (n % div === 0)
            return [
                ...divisors(div),
                ...divisors(n / div)
            ].sort((a, b) => a - b)
    return [n]
}

function merge(div1: number[], div2: number[]) {
    const div = []
    while (div1.length && div2.length) {
        const d1 = div1[0]
        const d2 = div2[0]
        if (d1 < d2)
            div.push(div1.shift())
        else if (d2 < d1)
            div.push(div2.shift())
        else
            div.push((div1.shift(), div2.shift()))
    }
    return [...div, ...div1, ...div2]
}

const seq = (n: number) => n ? [...seq(n - 1), n] : []

const ints = seq(20)
const divs = ints.map(divisors)
const merged = divs.reduce(merge)
const mul = merged.reduce((a, b) => a * b)

console.log(mul)
