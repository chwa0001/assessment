import {
  useContext,
  useState
} from 'react'
import { Routes as Router, Route, Navigate, Outlet, useLocation} from 'react-router-dom'
import { AppContext } from './context/AppContext'
import Home from './components/home';
import Home2 from './components/home2';

const PrivateRoutes = () => {
  const { is_auth } = useContext(AppContext);
  const location = useLocation();
  const [isLoading] = useState(true);

  if (isLoading) return <Home/>;
  if (!is_auth) return <Navigate to='/sign-in' replace state={{from:location}}/>;
  return <Outlet/>;
}

const Routes = () => {
  return (
    <Router>      
      <Route element={<PrivateRoutes/>}>
        <Route path='/' element={<Home />}>
            <Route path="test" element={<Home/>} />
        </Route>
      </Route>
      <Route path="/sign-in" element={<Home2/>} />
      <Route path="*" element={<Navigate to="/" replace />} />
    </Router>
  )
}

export default Routes
