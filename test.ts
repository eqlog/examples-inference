let k: number = 5;

function asdf() {
  if (true) { 
    return 'xyz';
  } else {
    return 'abc';
  }
}

function void_func() {
  let asdffff = 5;
  while (true) {
    let fkljskljdslk = 'asd';
  }
}

function id_number(x) {
  if (true) {
    return x;
  } else {
    return k;
  }
}

// The identity function returns its argument.
function id(y) {
  return y;
}

let the_five = id(5);
let the_xyz = id('xyz');

function id2(z) {
  return id2(id)(z);
}

function id3(u) {
  if (true) {
    return u;
  } else {
    return id3(id3)(u);
  }
}

id2(5);
