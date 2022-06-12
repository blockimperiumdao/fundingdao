import {
    applicant_countries,
    application_types,
    category_types,
    application_status_types
} from "../../../utils";

export const grantApplicationColumns = [

    { 
      field: "id", 
      headerName: "ID" 
    },
    { 
      field: "name", 
      headerName: "Name", 
      width: 150, 
      editable: true 
    },
    {
      field: "application_type",
      headerName: "Type",
      type: "singleSelect",
      valueOptions: ({ row }) => application_types,
      editable: true,
      width: 150,
    },
    {
      field: "grant_category",
      headerName: "Category",
      type: "singleSelect",
      valueOptions: ({ row }) => category_types,
      editable: true,
      width: 150,
    },
    {
      field: "description",
      headerName: "Description",
      width: 150,
    },
    {
      field: "applicant_first_name",
      headerName: "First Name",
    },
    {
      field: "applicant_last_name",
      headerName: "Last Name",
    },
    {
      field: "project_pitch_video_url",
      headerName: "Pitch Video",
      width: 150,
    },
    {
      field: "project_pitch_deck_url",
      headerName: "Pitch Deck",
      width: 150,
    },
    {
      field: "project_website",
      headerName: "Website",
      width: 150,
    },
    {
      field: "team_size",
      headerName: "Team Size",
      type: "number",
      editable: true,
    },
    {
      field: "application_status",
      headerName: "Status",
      type: "singleSelect",
      valueOptions: ({ row }) => application_status_types,
      editable: true,
      width: 150,
    },    

  ];

export const createGrantApplicationRow = () => {
    return {
        id: "",
        name: "Project Name",
        application_type: application_types[0],
        grant_category: category_types[0],
        description: "Description",
        applicant_first_name: "First Name",
        applicant_last_name: "Last Name",
        project_pitch_video_url: "Pitch Video URL",
        project_pitch_deck_url: "Pitch Deck URL",
        project_website: "Project website",
        team_size: 0,
        application_status: application_status_types[0],
    };    
}; 