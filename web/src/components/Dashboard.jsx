import * as React from 'react';

import { Stack, Box, Drawer } from '@mui/material';
import MenuBar from './MenuBar';
import SideBar from './SideBar';
import { Route, Routes } from 'react-router-dom';
import Copyright from './Copyright';
import BlockchainGrid from './grids/BlockchainGrid';

import { grantApplicationColumns, createGrantApplicationRow } from './grids/applicants/ApplicantGrid.js';

export default function Dashboard(props) {
    return (
        <Box bgcolor={"background.default"} color={"text.primary"}>
            <MenuBar />
            <Stack direction="row" spacing={2} justifyContent="space-between">
                <SideBar />

                <Routes>
                    <Route path="/" element={<Copyright/>} />
                    <Route path="applicants" element={<BlockchainGrid title="Grant Applicants Grid" 
                                                                columns={grantApplicationColumns}
                                                                dataFunction="get_grant_applications"
                                                                updateFunction="upsert_grant_application"
                                                                newRowFunction={createGrantApplicationRow()} />} />
                </Routes>

            </Stack>
        </Box>
    );
}