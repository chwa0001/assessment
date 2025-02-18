import { createContext, ReactNode, useState } from 'react';
import AppInfo from "../types/AppInfo";

type Props = {
  children?: ReactNode;
}

const initialValue = {
  id: -1,
  is_auth: false,
  backdrop: false,
  setID: () => {},
  setAuth: () => {},
  setBackdrop: () => {},
}

const AppContext = createContext<AppInfo>(initialValue)

const AppProvider = ({children}: Props) => {
  //Initializing an auth state with false value (unauthenticated)
  const [id, setID] = useState(initialValue.id);
  const [is_auth, setAuth] = useState(initialValue.is_auth);
  const [backdrop, setBackdrop] = useState<boolean>(initialValue.backdrop);


  return (
    <AppContext.Provider value={{id, is_auth, backdrop, setID, setAuth, setBackdrop}}>
      {children}
    </AppContext.Provider>
  )
}

export {  AppContext, AppProvider }