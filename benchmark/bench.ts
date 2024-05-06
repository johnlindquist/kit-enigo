import b from 'benny'

function add(a: number) {
  return a + 100
}

async function run() {}

run().catch((e) => {
  console.error(e)
})
