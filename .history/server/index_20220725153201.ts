import { opine } from 'https://deno.land/x/opine@2.2.0/mod.ts'
import { opineCors } from "https://deno.land/x/cors/mod.ts";

const app = opine()

app.use(opineCors())

app.get('/api', (req, res) => {
  res.json({ message: 'yew and deno' })
})

app.listen(3000, () => console.log('serving on port: 3000'))