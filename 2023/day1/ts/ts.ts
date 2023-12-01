const val = "lahdfadhad1two"

//from back
for (let i = val.length - 1; i >= 0; i--) {
    if (!isNaN(parseInt(val[i]))) {
    output += val[i];
    }
    wordNums.forEach((numerical, key) => {
    const wordLen: number = key.length;
    if (val.slice(i, i + wordLen) == key) {
        output += numerical;
    }
    });
    if (output.length == 2) break;
}