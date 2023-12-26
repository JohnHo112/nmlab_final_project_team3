
import "./App.css";
import { Outlet} from "react-router-dom";
import iota_logo from './images/IOTA_Logo_White.png'
import Nav from 'react-bootstrap/Nav'


function App() {
  return (
    <div className="App">
        <Nav>
          <Nav.Item>
            <Nav.Link href="/homepage">
              <img src={iota_logo} width={204.43} height={80} alt="IOTA LOGO" />
            </Nav.Link>
          </Nav.Item>
        </Nav>
    
      <div id="detail">
        <Outlet />
      </div>

    </div>
  );
}

export default App;
