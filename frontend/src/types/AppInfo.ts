export default interface AppInfo {
  id:number;
  is_auth: boolean;
  backdrop:boolean;
  setID: (newState: number) => void;
  setAuth: (newState: boolean) => void;
  setBackdrop: (newState: boolean) => void;
}
