function insertionSort(arr) {
  for (let i = 1; i < arr.length; i++) {
    let current = arr[i];
    let j = i - 1;

    while (j >= 0 && arr[j] > current) {
      arr[j + 1] = arr[j];
      j--;
    }

    arr[j + 1] = current;
  }
}

let data = [];
for (let i = 0; i < 10; i++) {
  data.push(Math.floor(Math.random() * 1000));
}

console.log(data);
insertionSort(data);
console.log(data);
