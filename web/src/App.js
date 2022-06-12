import * as React from 'react';

import Dashboard from './components/Dashboard';
import CssBaseline from '@mui/material/CssBaseline';
import Box from '@mui/material/Box';
import { BrowserRouter as Router, Link, Route, Switch } from 'react-router-dom';

import LoginScreen from './components/LoginScreen';

import { createTheme } from '@mui/material';
import { ThemeProvider } from '@emotion/react';

export default function App() {

  const [mode, setMode] = React.useState("light");

  const darkTheme = createTheme({
    palette: {
      mode: mode
    }
  });

  // if we haven't signed in, show the login screen
  //
  if (window.walletConnection.isSignedIn()) 
  {

    // if we are logged in show the dashboard
    return (
      <Router>
        <ThemeProvider theme={darkTheme}>
          <Box bgcolor={"background.default"} color={"text.primary"}>
            <Dashboard/>
          </Box>
        </ThemeProvider>
      </Router>
    )    
  }
  else
  {
    return (
        <Box>
          <CssBaseline />

          <LoginScreen />
        </Box>
    );
  }
}