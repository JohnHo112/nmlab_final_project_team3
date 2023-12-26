const express = require('express');
const cors = require('cors');
import routes from './routes'
const fileUpload = require('express-fileupload');

const app = express()

app.use(fileUpload())
app.use(cors())
app.use(express.json())

routes(app)
const port = process.env.PORT || 4000

app.listen(port, () => console.log(`Example app listening on port ${port}!`))
