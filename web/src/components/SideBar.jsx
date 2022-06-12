import * as React from 'react';

import { List, ListItem, ListItemButton, ListItemIcon, ListItemText, Box } from '@mui/material';
import { Face } from '@mui/icons-material';
import { useNavigate } from "react-router-dom";
 
export default function SideBar() {
    const navigate = useNavigate();

    return (

        <Box postion="fixed" bgcolor={"background.default"} color={"text.primary"}>

            <List >
                <ListItem disablePadding>
                    <ListItemButton onClick={() => navigate('/applicants')}>
                        <ListItemIcon>
                            <Face />
                        </ListItemIcon>

                        <ListItemText primary="Applicants" secondary="Grant Applicants"/>
                    </ListItemButton>
                </ListItem>


            </List>
        </Box>
    );
}