import * as React from 'react';

import { logout } from '../utils';

import Typography from '@mui/material/Typography';
import { AppBar, styled, Toolbar, Button, Box } from '@mui/material';

const StyledToolbar = styled(Toolbar)({
    display:"flex",
    justifyContent:"space-between"
});


export default function MenuBar(props) {
    return (
        <Box sx={{ flexGrow: 1 }} bgcolor={"background.default"} color={"text.primary"}>
            <AppBar position="sticky">
                <StyledToolbar>
                    <Typography variant="h6" sx= {{display:{xs:"none", sm:"block"}}}>
                        FundingDAO
                    </Typography>
                    <Button color="inherit" onClick={logout}>Logout</Button>
                </StyledToolbar>
            </AppBar>
        </Box>

    );
}