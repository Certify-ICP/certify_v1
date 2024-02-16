import { useState } from 'react'
import { BrowserRouter, Route, Routes } from 'react-router-dom'
import Home from './landingpage'
import Upload from './uploadpage'
import Org from './universityupload'


function App() {
  const [count, setCount] = useState(0)

  return (
    <div>
     <BrowserRouter>
     <Routes>
      <Route path='/'element={<Home />}> </Route>
      <Route index element={<Home />} ></Route>
      <Route path='upload' element={<Upload />}></Route>
      <Route path='universitymodal' element={<Org />}></Route>
     </Routes>
     </BrowserRouter>
    </div>
  )
}

export default App
