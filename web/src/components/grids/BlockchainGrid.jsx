import * as React from "react";
import { useState, useEffect } from "react";

import { Alert, Box, Button, Snackbar, Stack, Typography } from "@mui/material";
import { GridToolbar, DataGrid } from "@mui/x-data-grid";
import { styled } from "@mui/system";

const StyledBox = styled(Box)({
  paddingTop: 10,
  paddingRight: 10,
});

async function data( dataFunction ) {

  console.log("Executing dataFunction " + JSON.stringify(dataFunction) + " on the contract");

  let response = [];

  let rowData = await window["contract"][ dataFunction ]({
    from_index: 0,
    limit: 10,
  });

  rowData.forEach((item) => {
    response.push(item[1]);
  });

  return response;
}

async function update( updateFunction, mutatedRowData ) {
    console.log("Executing updateFuction " + JSON.stringify(updateFunction) + " on the contract");

    let response = await window["contract"][updateFunction](mutatedRowData );

    return response;
}

export default function BlockchainGrid(props) 
{
  const [tableData, setTableData] = useState([]);
  const [snackbar, setSnackbar] = React.useState(null);

  useEffect(() => {
    data(props.dataFunction).then((response) => setTableData(response));
  }, [props.dataFunction]);

  const handleAddRow = () => {
    setTableData((prevRows) => [...prevRows, props.newRowFunction]);
  };

  const handleRowEditCommit = () => {
    console.log("Edit commit");
  }
  
  const handleRowEditStop = () => {
    console.log("Edit stop");
  }

  const useMutateRow = () => {

    return React.useCallback(
      (mutatedRowData) => 
        new Promise((resolve, reject) =>
          setTimeout(() => {       

            const response = update( props.updateFunction, mutatedRowData );

            console.log("Received response from upsert " + JSON.stringify(response));
            resolve({ ...mutatedRowData })
          }, 200)
        ),
      []
    );
  };

  const mutateRow = useMutateRow();

  const processRowUpdate = React.useCallback(
    async (newRow) => {

      console.log("Upsert " + JSON.stringify(newRow) );

      // Make the HTTP request to save in the backend
      const response = await mutateRow(newRow).then(setSnackbar({ children: "Data saved successfully...", severity: "success" }));
      return response;
    },
    [mutateRow]
  );

  const handleCloseSnackbar = () => setSnackbar(null);

  const handleProcessRowUpdateError = React.useCallback((error) => {
    setSnackbar({ children: error.message, severity: "error" });
  }, []);

  return (
    <StyledBox flex={4}>
      <Typography variant="h6">{props.title}</Typography>

      <Stack
        sx={{ width: "100%", mb: 1 }}
        direction="row"
        alignItems="flex-start"
        columnGap={1}
      >
        <Button size="small" onClick={handleAddRow}>
          Add a row
        </Button>
      </Stack>
      <DataGrid
        rows={tableData}
        columns={props.columns}
        components={{ Toolbar: GridToolbar }}
        checkboxSelection
        disableSelectionOnClick
        experimentalFeatures={{ newEditingApi: true }}
        onRowEditCommit={handleRowEditCommit}
        onRowEditStop={handleRowEditStop}
        processRowUpdate={processRowUpdate}
        onProcessRowUpdateError={handleProcessRowUpdateError}
        editMode="row"
      />
      {!!snackbar && (
        <Snackbar
          open
          anchorOrigin={{ vertical: "bottom", horizontal: "center" }}
          onClose={handleCloseSnackbar}
          autoHideDuration={6000}
        >
          <Alert {...snackbar} onClose={handleCloseSnackbar} />
        </Snackbar>
      )}
    </StyledBox>
  );
}
