mgg[100,180];
mjj[70,190];
mtot[200,1600];
ttHTagger[-1,1];
CMS_hhbbgg_13TeV_mgg_bkg_slope1_cat0[0.9, -1, 10];
CMS_hhbbgg_13TeV_mgg_bkg_slope2_cat0[0.5, -1, 10];
CMS_hhbbgg_13TeV_mgg_bkg_slope3_cat0[0.5, -1, 10];

mgg_sig_m0_cat0[125., 122, 127];
mgg_sig_sigma_cat0[1.0, 0.1, 3.0];
mgg_sig_alpha1_cat0[1.0, 0.05, 10.0];
mgg_sig_n1_cat0[2.0, 0.1, 10.0];
mgg_sig_alpha2_cat0[1.0, 0.05, 10.0];
mgg_sig_n2_cat0[2.0, 0.1, 10.0];
mggSig_cat0 = RooDoubleCB(mgg, mgg_sig_m0_cat0, mgg_sig_sigma_cat0, mgg_sig_alpha1_cat0, mgg_sig_n1_cat0, mgg_sig_alpha2_cat0, mgg_sig_n2_cat0);

mgg_hig_m0_ggh_cat0[124.2, 123, 125];
mgg_hig_sigma_ggh_cat0[2.0, 0.1, 3.0];
mgg_hig_alpha1_ggh_cat0[1.0, 0.05, 10.0];
mgg_hig_n1_ggh_cat0[2.0, 0.1, 10.0];
mgg_hig_alpha2_ggh_cat0[1.0, 0.05, 10.0];
mgg_hig_n2_ggh_cat0[2.0, 0.1, 10.0];
mggHig_ggh_cat0 = RooDoubleCB(mgg, mgg_hig_m0_ggh_cat0, mgg_hig_sigma_ggh_cat0, mgg_hig_alpha1_ggh_cat0, mgg_hig_n1_ggh_cat0, mgg_hig_alpha2_ggh_cat0, mgg_hig_n2_ggh_cat0);

mgg_hig_m0_tth_cat0[124.2, 123, 125];
mgg_hig_sigma_tth_cat0[2.0, 0.1, 3.0];
mgg_hig_alpha1_tth_cat0[1.0, 0.05, 10.0];
mgg_hig_n1_tth_cat0[2.0, 0.1, 10.0];
mgg_hig_alpha2_tth_cat0[1.0, 0.05, 10.0];
mgg_hig_n2_tth_cat0[2.0, 0.1, 10.0];
mggHig_tth_cat0 = RooDoubleCB(mgg, mgg_hig_m0_tth_cat0, mgg_hig_sigma_tth_cat0, mgg_hig_alpha1_tth_cat0, mgg_hig_n1_tth_cat0, mgg_hig_alpha2_tth_cat0, mgg_hig_n2_tth_cat0);

mgg_hig_m0_vh_cat0[124.2, 123, 125];
mgg_hig_sigma_vh_cat0[2.0, 0.1, 3.0];
mgg_hig_alpha1_vh_cat0[1.0, 0.05, 10.0];
mgg_hig_n1_vh_cat0[2.0, 0.1, 10.0];
mgg_hig_alpha2_vh_cat0[1.0, 0.05, 10.0];
mgg_hig_n2_vh_cat0[2.0, 0.1, 10.0];
mggHig_vh_cat0 = RooDoubleCB(mgg, mgg_hig_m0_vh_cat0, mgg_hig_sigma_vh_cat0, mgg_hig_alpha1_vh_cat0, mgg_hig_n1_vh_cat0, mgg_hig_alpha2_vh_cat0, mgg_hig_n2_vh_cat0);

mgg_hig_m0_vbf_cat0[124.2, 123, 125];
mgg_hig_sigma_vbf_cat0[2.0, 0.1, 3.0];
mgg_hig_alpha1_vbf_cat0[1.0, 0.05, 10.0];
mgg_hig_n1_vbf_cat0[2.0, 0.1, 10.0];
mgg_hig_alpha2_vbf_cat0[1.0, 0.05, 10.0];
mgg_hig_n2_vbf_cat0[2.0, 0.1, 10.0];
mggHig_vbf_cat0 = RooDoubleCB(mgg, mgg_hig_m0_vbf_cat0, mgg_hig_sigma_vbf_cat0, mgg_hig_alpha1_vbf_cat0, mgg_hig_n1_vbf_cat0, mgg_hig_alpha2_vbf_cat0, mgg_hig_n2_vbf_cat0);

mgg_hig_m0_bbh_cat0[124.2, 123, 125];
mgg_hig_sigma_bbh_cat0[2.0, 0.1, 3.0];
mgg_hig_alpha1_bbh_cat0[1.0, 0.05, 10.0];
mgg_hig_n1_bbh_cat0[2.0, 0.1, 10.0];
mgg_hig_alpha2_bbh_cat0[1.0, 0.05, 10.0];
mgg_hig_n2_bbh_cat0[2.0, 0.1, 10.0];
mggHig_bbh_cat0 = RooDoubleCB(mgg, mgg_hig_m0_bbh_cat0, mgg_hig_sigma_bbh_cat0, mgg_hig_alpha1_bbh_cat0, mgg_hig_n1_bbh_cat0, mgg_hig_alpha2_bbh_cat0, mgg_hig_n2_bbh_cat0);


mjj_sig_m0_cat0[110.0, 99, 140];
mjj_sig_sigma_cat0[10.0, 1.0, 60.0];
mjj_sig_alpha1_cat0[1.0, 0.05, 10.0];
mjj_sig_n1_cat0[2.0, 0.1, 10.0];
mjj_sig_alpha2_cat0[1.0, 0.05, 10.0];
mjj_sig_n2_cat0[2.0, 0.1, 10.0];
mjjSig_cat0 = RooDoubleCB(mjj, mjj_sig_m0_cat0, mjj_sig_sigma_cat0, mjj_sig_alpha1_cat0, mjj_sig_n1_cat0, mjj_sig_alpha2_cat0, mjj_sig_n2_cat0);

CMS_hhbbgg_13TeV_mjj_bkg_slope1_cat0[0.9, -1, 10];
CMS_hhbbgg_13TeV_mjj_bkg_slope2_cat0[0.5, -1, 10];
CMS_hhbbgg_13TeV_mjj_bkg_slope3_cat0[0.5, -1, 10];

mjj_hig_slope1_ggh_cat0[0.1, 0, 10];
mjj_hig_slope2_ggh_cat0[0.1, 0, 10];
mjj_hig_slope3_ggh_cat0[0.1, 0, 10];

mjj_hig_slope1_vbf_cat0[0.1, 0, 10];
mjj_hig_slope2_vbf_cat0[0.1, 0, 10];
mjj_hig_slope3_vbf_cat0[0.1, 0, 10];

mjj_hig_m0_tth_cat0[100, 70, 190];
mjj_hig_sigma_tth_cat0[50, 10, 100];
mjj_hig_alpha1_tth_cat0[1.0, 0.01, 10];
mjj_hig_n1_tth_cat0[1, 0.01, 10];
mjj_hig_alpha2_tth_cat0[1.0, 0.01, 10];
mjj_hig_n2_tth_cat0[1, 0.01, 10];
mjjHig_tth_cat0 = RooDoubleCB(mjj, mjj_hig_m0_tth_cat0, mjj_hig_sigma_tth_cat0, mjj_hig_alpha1_tth_cat0, mjj_hig_n1_tth_cat0, mjj_hig_alpha2_tth_cat0, mjj_hig_n2_tth_cat0);

mjj_hig_m0_vh_cat0[100, 70, 190];
mjj_hig_sigma_vh_cat0[50, 10, 100];
mjj_hig_alpha1_vh_cat0[1.0, 0.01, 10];
mjj_hig_n1_vh_cat0[1, 0.01, 10];
mjj_hig_alpha2_vh_cat0[1.0, 0.01, 10];
mjj_hig_n2_vh_cat0[1, 0.01, 10];
mjjHig_vh_cat0 = RooDoubleCB(mjj, mjj_hig_m0_vh_cat0, mjj_hig_sigma_vh_cat0, mjj_hig_alpha1_vh_cat0, mjj_hig_n1_vh_cat0, mjj_hig_alpha2_vh_cat0, mjj_hig_n2_vh_cat0);

mjj_hig_m0_bbh_cat0[100, 10, 180];
mjj_hig_sigma_bbh_cat0[50, 1.0, 100];
mjj_hig_alpha1_bbh_cat0[1.0, 0.01, 10];
mjj_hig_n1_bbh_cat0[1, 0.01, 10];
mjj_hig_alpha2_bbh_cat0[1.0, 0.01, 10];
mjj_hig_n2_bbh_cat0[1, 0.01, 10];
mjjHig_bbh_cat0 = RooDoubleCB(mjj, mjj_hig_m0_bbh_cat0, mjj_hig_sigma_bbh_cat0, mjj_hig_alpha1_bbh_cat0, mjj_hig_n1_bbh_cat0, mjj_hig_alpha2_bbh_cat0, mjj_hig_n2_bbh_cat0);CMS_hhbbgg_13TeV_mgg_bkg_slope1_cat1[0.9, -1, 10];
CMS_hhbbgg_13TeV_mgg_bkg_slope2_cat1[0.5, -1, 10];
CMS_hhbbgg_13TeV_mgg_bkg_slope3_cat1[0.5, -1, 10];

mgg_sig_m0_cat1[125., 122, 127];
mgg_sig_sigma_cat1[1.0, 0.1, 3.0];
mgg_sig_alpha1_cat1[1.0, 0.05, 10.0];
mgg_sig_n1_cat1[2.0, 0.1, 10.0];
mgg_sig_alpha2_cat1[1.0, 0.05, 10.0];
mgg_sig_n2_cat1[2.0, 0.1, 10.0];
mggSig_cat1 = RooDoubleCB(mgg, mgg_sig_m0_cat1, mgg_sig_sigma_cat1, mgg_sig_alpha1_cat1, mgg_sig_n1_cat1, mgg_sig_alpha2_cat1, mgg_sig_n2_cat1);

mgg_hig_m0_ggh_cat1[124.2, 123, 125];
mgg_hig_sigma_ggh_cat1[2.0, 0.1, 3.0];
mgg_hig_alpha1_ggh_cat1[1.0, 0.05, 10.0];
mgg_hig_n1_ggh_cat1[2.0, 0.1, 10.0];
mgg_hig_alpha2_ggh_cat1[1.0, 0.05, 10.0];
mgg_hig_n2_ggh_cat1[2.0, 0.1, 10.0];
mggHig_ggh_cat1 = RooDoubleCB(mgg, mgg_hig_m0_ggh_cat1, mgg_hig_sigma_ggh_cat1, mgg_hig_alpha1_ggh_cat1, mgg_hig_n1_ggh_cat1, mgg_hig_alpha2_ggh_cat1, mgg_hig_n2_ggh_cat1);

mgg_hig_m0_tth_cat1[124.2, 123, 125];
mgg_hig_sigma_tth_cat1[2.0, 0.1, 3.0];
mgg_hig_alpha1_tth_cat1[1.0, 0.05, 10.0];
mgg_hig_n1_tth_cat1[2.0, 0.1, 10.0];
mgg_hig_alpha2_tth_cat1[1.0, 0.05, 10.0];
mgg_hig_n2_tth_cat1[2.0, 0.1, 10.0];
mggHig_tth_cat1 = RooDoubleCB(mgg, mgg_hig_m0_tth_cat1, mgg_hig_sigma_tth_cat1, mgg_hig_alpha1_tth_cat1, mgg_hig_n1_tth_cat1, mgg_hig_alpha2_tth_cat1, mgg_hig_n2_tth_cat1);

mgg_hig_m0_vh_cat1[124.2, 123, 125];
mgg_hig_sigma_vh_cat1[2.0, 0.1, 3.0];
mgg_hig_alpha1_vh_cat1[1.0, 0.05, 10.0];
mgg_hig_n1_vh_cat1[2.0, 0.1, 10.0];
mgg_hig_alpha2_vh_cat1[1.0, 0.05, 10.0];
mgg_hig_n2_vh_cat1[2.0, 0.1, 10.0];
mggHig_vh_cat1 = RooDoubleCB(mgg, mgg_hig_m0_vh_cat1, mgg_hig_sigma_vh_cat1, mgg_hig_alpha1_vh_cat1, mgg_hig_n1_vh_cat1, mgg_hig_alpha2_vh_cat1, mgg_hig_n2_vh_cat1);

mgg_hig_m0_vbf_cat1[124.2, 123, 125];
mgg_hig_sigma_vbf_cat1[2.0, 0.1, 3.0];
mgg_hig_alpha1_vbf_cat1[1.0, 0.05, 10.0];
mgg_hig_n1_vbf_cat1[2.0, 0.1, 10.0];
mgg_hig_alpha2_vbf_cat1[1.0, 0.05, 10.0];
mgg_hig_n2_vbf_cat1[2.0, 0.1, 10.0];
mggHig_vbf_cat1 = RooDoubleCB(mgg, mgg_hig_m0_vbf_cat1, mgg_hig_sigma_vbf_cat1, mgg_hig_alpha1_vbf_cat1, mgg_hig_n1_vbf_cat1, mgg_hig_alpha2_vbf_cat1, mgg_hig_n2_vbf_cat1);

mgg_hig_m0_bbh_cat1[124.2, 123, 125];
mgg_hig_sigma_bbh_cat1[2.0, 0.1, 3.0];
mgg_hig_alpha1_bbh_cat1[1.0, 0.05, 10.0];
mgg_hig_n1_bbh_cat1[2.0, 0.1, 10.0];
mgg_hig_alpha2_bbh_cat1[1.0, 0.05, 10.0];
mgg_hig_n2_bbh_cat1[2.0, 0.1, 10.0];
mggHig_bbh_cat1 = RooDoubleCB(mgg, mgg_hig_m0_bbh_cat1, mgg_hig_sigma_bbh_cat1, mgg_hig_alpha1_bbh_cat1, mgg_hig_n1_bbh_cat1, mgg_hig_alpha2_bbh_cat1, mgg_hig_n2_bbh_cat1);


mjj_sig_m0_cat1[110.0, 99, 140];
mjj_sig_sigma_cat1[10.0, 1.0, 60.0];
mjj_sig_alpha1_cat1[1.0, 0.05, 10.0];
mjj_sig_n1_cat1[2.0, 0.1, 10.0];
mjj_sig_alpha2_cat1[1.0, 0.05, 10.0];
mjj_sig_n2_cat1[2.0, 0.1, 5.0];
mjjSig_cat1 = RooDoubleCB(mjj, mjj_sig_m0_cat1, mjj_sig_sigma_cat1, mjj_sig_alpha1_cat1, mjj_sig_n1_cat1, mjj_sig_alpha2_cat1, mjj_sig_n2_cat1);

CMS_hhbbgg_13TeV_mjj_bkg_slope1_cat1[0.9, -1, 10];
CMS_hhbbgg_13TeV_mjj_bkg_slope2_cat1[0.5, -1, 10];
CMS_hhbbgg_13TeV_mjj_bkg_slope3_cat1[0.5, -1, 10];

mjj_hig_slope1_ggh_cat1[0.1, 0, 10];
mjj_hig_slope2_ggh_cat1[0.1, 0, 10];
mjj_hig_slope3_ggh_cat1[0.1, 0, 10];

mjj_hig_slope1_vbf_cat1[0.1, 0, 10];
mjj_hig_slope2_vbf_cat1[0.1, 0, 10];
mjj_hig_slope3_vbf_cat1[0.1, 0, 10];

mjj_hig_m0_tth_cat1[100, 70, 190];
mjj_hig_sigma_tth_cat1[50, 10, 100];
mjj_hig_alpha1_tth_cat1[1.0, 0.01, 10];
mjj_hig_n1_tth_cat1[1, 0.01, 10];
mjj_hig_alpha2_tth_cat1[1.0, 0.01, 10];
mjj_hig_n2_tth_cat1[1, 0.01, 10];
mjjHig_tth_cat1 = RooDoubleCB(mjj, mjj_hig_m0_tth_cat1, mjj_hig_sigma_tth_cat1, mjj_hig_alpha1_tth_cat1, mjj_hig_n1_tth_cat1, mjj_hig_alpha2_tth_cat1, mjj_hig_n2_tth_cat1);

mjj_hig_m0_vh_cat1[100, 70, 190];
mjj_hig_sigma_vh_cat1[50, 10, 100];
mjj_hig_alpha1_vh_cat1[1.0, 0.01, 10];
mjj_hig_n1_vh_cat1[1, 0.01, 10];
mjj_hig_alpha2_vh_cat1[1.0, 0.01, 10];
mjj_hig_n2_vh_cat1[1, 0.01, 10];
mjjHig_vh_cat1 = RooDoubleCB(mjj, mjj_hig_m0_vh_cat1, mjj_hig_sigma_vh_cat1, mjj_hig_alpha1_vh_cat1, mjj_hig_n1_vh_cat1, mjj_hig_alpha2_vh_cat1, mjj_hig_n2_vh_cat1);

mjj_hig_m0_bbh_cat1[100, 10, 180];
mjj_hig_sigma_bbh_cat1[50, 1.0, 100];
mjj_hig_alpha1_bbh_cat1[1.0, 0.01, 10];
mjj_hig_n1_bbh_cat1[1, 0.01, 10];
mjj_hig_alpha2_bbh_cat1[1.0, 0.01, 10];
mjj_hig_n2_bbh_cat1[1, 0.01, 10];
mjjHig_bbh_cat1 = RooDoubleCB(mjj, mjj_hig_m0_bbh_cat1, mjj_hig_sigma_bbh_cat1, mjj_hig_alpha1_bbh_cat1, mjj_hig_n1_bbh_cat1, mjj_hig_alpha2_bbh_cat1, mjj_hig_n2_bbh_cat1);CMS_hhbbgg_13TeV_mgg_bkg_slope1_cat2[0.9, -1, 10];
CMS_hhbbgg_13TeV_mgg_bkg_slope2_cat2[0.5, -1, 10];
CMS_hhbbgg_13TeV_mgg_bkg_slope3_cat2[0.5, -1, 10];

mgg_sig_m0_cat2[125., 122, 127];
mgg_sig_sigma_cat2[1.0, 0.1, 3.0];
mgg_sig_alpha1_cat2[1.0, 0.05, 10.0];
mgg_sig_n1_cat2[2.0, 0.1, 10.0];
mgg_sig_alpha2_cat2[1.0, 0.05, 10.0];
mgg_sig_n2_cat2[2.0, 0.1, 10.0];
mggSig_cat2 = RooDoubleCB(mgg, mgg_sig_m0_cat2, mgg_sig_sigma_cat2, mgg_sig_alpha1_cat2, mgg_sig_n1_cat2, mgg_sig_alpha2_cat2, mgg_sig_n2_cat2);

mgg_hig_m0_ggh_cat2[124.2, 123, 125];
mgg_hig_sigma_ggh_cat2[2.0, 0.1, 3.0];
mgg_hig_alpha1_ggh_cat2[1.0, 0.05, 10.0];
mgg_hig_n1_ggh_cat2[2.0, 0.1, 10.0];
mgg_hig_alpha2_ggh_cat2[1.0, 0.05, 10.0];
mgg_hig_n2_ggh_cat2[2.0, 0.1, 10.0];
mggHig_ggh_cat2 = RooDoubleCB(mgg, mgg_hig_m0_ggh_cat2, mgg_hig_sigma_ggh_cat2, mgg_hig_alpha1_ggh_cat2, mgg_hig_n1_ggh_cat2, mgg_hig_alpha2_ggh_cat2, mgg_hig_n2_ggh_cat2);

mgg_hig_m0_tth_cat2[124.2, 123, 125];
mgg_hig_sigma_tth_cat2[2.0, 0.1, 3.0];
mgg_hig_alpha1_tth_cat2[1.0, 0.05, 10.0];
mgg_hig_n1_tth_cat2[2.0, 0.1, 10.0];
mgg_hig_alpha2_tth_cat2[1.0, 0.05, 10.0];
mgg_hig_n2_tth_cat2[2.0, 0.1, 10.0];
mggHig_tth_cat2 = RooDoubleCB(mgg, mgg_hig_m0_tth_cat2, mgg_hig_sigma_tth_cat2, mgg_hig_alpha1_tth_cat2, mgg_hig_n1_tth_cat2, mgg_hig_alpha2_tth_cat2, mgg_hig_n2_tth_cat2);

mgg_hig_m0_vh_cat2[124.2, 123, 125];
mgg_hig_sigma_vh_cat2[2.0, 0.1, 3.0];
mgg_hig_alpha1_vh_cat2[1.0, 0.05, 10.0];
mgg_hig_n1_vh_cat2[2.0, 0.1, 10.0];
mgg_hig_alpha2_vh_cat2[1.0, 0.05, 10.0];
mgg_hig_n2_vh_cat2[2.0, 0.1, 10.0];
mggHig_vh_cat2 = RooDoubleCB(mgg, mgg_hig_m0_vh_cat2, mgg_hig_sigma_vh_cat2, mgg_hig_alpha1_vh_cat2, mgg_hig_n1_vh_cat2, mgg_hig_alpha2_vh_cat2, mgg_hig_n2_vh_cat2);

mgg_hig_m0_vbf_cat2[124.2, 123, 125];
mgg_hig_sigma_vbf_cat2[2.0, 0.1, 3.0];
mgg_hig_alpha1_vbf_cat2[1.0, 0.05, 10.0];
mgg_hig_n1_vbf_cat2[2.0, 0.1, 10.0];
mgg_hig_alpha2_vbf_cat2[1.0, 0.05, 10.0];
mgg_hig_n2_vbf_cat2[2.0, 0.1, 10.0];
mggHig_vbf_cat2 = RooDoubleCB(mgg, mgg_hig_m0_vbf_cat2, mgg_hig_sigma_vbf_cat2, mgg_hig_alpha1_vbf_cat2, mgg_hig_n1_vbf_cat2, mgg_hig_alpha2_vbf_cat2, mgg_hig_n2_vbf_cat2);

mgg_hig_m0_bbh_cat2[124.2, 123, 125];
mgg_hig_sigma_bbh_cat2[2.0, 0.1, 3.0];
mgg_hig_alpha1_bbh_cat2[1.0, 0.05, 10.0];
mgg_hig_n1_bbh_cat2[2.0, 0.1, 10.0];
mgg_hig_alpha2_bbh_cat2[1.0, 0.05, 10.0];
mgg_hig_n2_bbh_cat2[2.0, 0.1, 10.0];
mggHig_bbh_cat2 = RooDoubleCB(mgg, mgg_hig_m0_bbh_cat2, mgg_hig_sigma_bbh_cat2, mgg_hig_alpha1_bbh_cat2, mgg_hig_n1_bbh_cat2, mgg_hig_alpha2_bbh_cat2, mgg_hig_n2_bbh_cat2);

mjj_sig_m0_cat2[110.0, 99, 140];
mjj_sig_sigma_cat2[10.0, 1.0, 60.0];
mjj_sig_alpha1_cat2[1.0, 0.05, 10.0];
mjj_sig_n1_cat2[2.0, 0.1, 10.0];
mjj_sig_alpha2_cat2[1.0, 0.05, 10.0];
mjj_sig_n2_cat2[2.0, 0.1, 5.0];
mjjSig_cat2 = RooDoubleCB(mjj, mjj_sig_m0_cat2, mjj_sig_sigma_cat2, mjj_sig_alpha1_cat2, mjj_sig_n1_cat2, mjj_sig_alpha2_cat2, mjj_sig_n2_cat2);

CMS_hhbbgg_13TeV_mjj_bkg_slope1_cat2[0.9, -1, 10];
CMS_hhbbgg_13TeV_mjj_bkg_slope2_cat2[0.5, -1, 10];
CMS_hhbbgg_13TeV_mjj_bkg_slope3_cat2[0.5, -1, 10];

mjj_hig_slope1_ggh_cat2[0.1, 0, 10];
mjj_hig_slope2_ggh_cat2[0.1, 0, 10];
mjj_hig_slope3_ggh_cat2[0.1, 0, 10];

mjj_hig_slope1_vbf_cat2[0.1, 0, 10];
mjj_hig_slope2_vbf_cat2[0.1, 0, 10];
mjj_hig_slope3_vbf_cat2[0.1, 0, 10];

mjj_hig_m0_tth_cat2[100, 70, 190];
mjj_hig_sigma_tth_cat2[50, 10, 100];
mjj_hig_alpha1_tth_cat2[1.0, 0.01, 10];
mjj_hig_n1_tth_cat2[1, 0.01, 10];
mjj_hig_alpha2_tth_cat2[1.0, 0.01, 10];
mjj_hig_n2_tth_cat2[1, 0.01, 10];
mjjHig_tth_cat2 = RooDoubleCB(mjj, mjj_hig_m0_tth_cat2, mjj_hig_sigma_tth_cat2, mjj_hig_alpha1_tth_cat2, mjj_hig_n1_tth_cat2, mjj_hig_alpha2_tth_cat2, mjj_hig_n2_tth_cat2);

mjj_hig_m0_vh_cat2[100, 70, 190];
mjj_hig_sigma_vh_cat2[50, 10, 100];
mjj_hig_alpha1_vh_cat2[1.0, 0.01, 10];
mjj_hig_n1_vh_cat2[1, 0.01, 10];
mjj_hig_alpha2_vh_cat2[1.0, 0.01, 10];
mjj_hig_n2_vh_cat2[1, 0.01, 10];
mjjHig_vh_cat2 = RooDoubleCB(mjj, mjj_hig_m0_vh_cat2, mjj_hig_sigma_vh_cat2, mjj_hig_alpha1_vh_cat2, mjj_hig_n1_vh_cat2, mjj_hig_alpha2_vh_cat2, mjj_hig_n2_vh_cat2);

mjj_hig_m0_bbh_cat2[100, 10, 180];
mjj_hig_sigma_bbh_cat2[50, 1.0, 100];
mjj_hig_alpha1_bbh_cat2[1.0, 0.01, 10];
mjj_hig_n1_bbh_cat2[1, 0.01, 10];
mjj_hig_alpha2_bbh_cat2[1.0, 0.01, 10];
mjj_hig_n2_bbh_cat2[1, 0.01, 10];
mjjHig_bbh_cat2 = RooDoubleCB(mjj, mjj_hig_m0_bbh_cat2, mjj_hig_sigma_bbh_cat2, mjj_hig_alpha1_bbh_cat2, mjj_hig_n1_bbh_cat2, mjj_hig_alpha2_bbh_cat2, mjj_hig_n2_bbh_cat2);CMS_hhbbgg_13TeV_mgg_bkg_slope1_cat3[0.9, -1, 10];
CMS_hhbbgg_13TeV_mgg_bkg_slope2_cat3[0.5, -1, 10];
CMS_hhbbgg_13TeV_mgg_bkg_slope3_cat3[0.5, -1, 10];

mgg_sig_m0_cat3[125., 122, 127];
mgg_sig_sigma_cat3[1.0, 0.1, 3.0];
mgg_sig_alpha1_cat3[1.0, 0.05, 10.0];
mgg_sig_n1_cat3[2.0, 0.1, 10.0];
mgg_sig_alpha2_cat3[1.0, 0.05, 10.0];
mgg_sig_n2_cat3[2.0, 0.1, 10.0];
mggSig_cat3 = RooDoubleCB(mgg, mgg_sig_m0_cat3, mgg_sig_sigma_cat3, mgg_sig_alpha1_cat3, mgg_sig_n1_cat3, mgg_sig_alpha2_cat3, mgg_sig_n2_cat3);

mgg_hig_m0_ggh_cat3[124.2, 123, 125];
mgg_hig_sigma_ggh_cat3[2.0, 0.1, 3.0];
mgg_hig_alpha1_ggh_cat3[1.0, 0.05, 10.0];
mgg_hig_n1_ggh_cat3[2.0, 0.1, 10.0];
mgg_hig_alpha2_ggh_cat3[1.0, 0.05, 10.0];
mgg_hig_n2_ggh_cat3[2.0, 0.1, 10.0];
mggHig_ggh_cat3 = RooDoubleCB(mgg, mgg_hig_m0_ggh_cat3, mgg_hig_sigma_ggh_cat3, mgg_hig_alpha1_ggh_cat3, mgg_hig_n1_ggh_cat3, mgg_hig_alpha2_ggh_cat3, mgg_hig_n2_ggh_cat3);

mgg_hig_m0_tth_cat3[124.2, 123, 125];
mgg_hig_sigma_tth_cat3[2.0, 0.1, 3.0];
mgg_hig_alpha1_tth_cat3[1.0, 0.05, 10.0];
mgg_hig_n1_tth_cat3[2.0, 0.1, 10.0];
mgg_hig_alpha2_tth_cat3[1.0, 0.05, 10.0];
mgg_hig_n2_tth_cat3[2.0, 0.1, 10.0];
mggHig_tth_cat3 = RooDoubleCB(mgg, mgg_hig_m0_tth_cat3, mgg_hig_sigma_tth_cat3, mgg_hig_alpha1_tth_cat3, mgg_hig_n1_tth_cat3, mgg_hig_alpha2_tth_cat3, mgg_hig_n2_tth_cat3);

mgg_hig_m0_vh_cat3[124.2, 123, 125];
mgg_hig_sigma_vh_cat3[2.0, 0.1, 3.0];
mgg_hig_alpha1_vh_cat3[1.0, 0.05, 10.0];
mgg_hig_n1_vh_cat3[2.0, 0.1, 10.0];
mgg_hig_alpha2_vh_cat3[1.0, 0.05, 10.0];
mgg_hig_n2_vh_cat3[2.0, 0.1, 10.0];
mggHig_vh_cat3 = RooDoubleCB(mgg, mgg_hig_m0_vh_cat3, mgg_hig_sigma_vh_cat3, mgg_hig_alpha1_vh_cat3, mgg_hig_n1_vh_cat3, mgg_hig_alpha2_vh_cat3, mgg_hig_n2_vh_cat3);

mgg_hig_m0_vbf_cat3[124.2, 123, 125];
mgg_hig_sigma_vbf_cat3[2.0, 0.1, 3.0];
mgg_hig_alpha1_vbf_cat3[1.0, 0.05, 10.0];
mgg_hig_n1_vbf_cat3[2.0, 0.1, 10.0];
mgg_hig_alpha2_vbf_cat3[1.0, 0.05, 10.0];
mgg_hig_n2_vbf_cat3[2.0, 0.1, 10.0];
mggHig_vbf_cat3 = RooDoubleCB(mgg, mgg_hig_m0_vbf_cat3, mgg_hig_sigma_vbf_cat3, mgg_hig_alpha1_vbf_cat3, mgg_hig_n1_vbf_cat3, mgg_hig_alpha2_vbf_cat3, mgg_hig_n2_vbf_cat3);

mgg_hig_m0_bbh_cat3[124.2, 123, 125];
mgg_hig_sigma_bbh_cat3[2.0, 0.1, 3.0];
mgg_hig_alpha1_bbh_cat3[1.0, 0.05, 10.0];
mgg_hig_n1_bbh_cat3[2.0, 0.1, 10.0];
mgg_hig_alpha2_bbh_cat3[1.0, 0.05, 10.0];
mgg_hig_n2_bbh_cat3[2.0, 0.1, 10.0];
mggHig_bbh_cat3 = RooDoubleCB(mgg, mgg_hig_m0_bbh_cat3, mgg_hig_sigma_bbh_cat3, mgg_hig_alpha1_bbh_cat3, mgg_hig_n1_bbh_cat3, mgg_hig_alpha2_bbh_cat3, mgg_hig_n2_bbh_cat3);

mjj_sig_m0_cat3[110.0, 99, 140];
mjj_sig_sigma_cat3[10.0, 1.0, 60.0];
mjj_sig_alpha1_cat3[1.0, 0.05, 10.0];
mjj_sig_n1_cat3[2.0, 0.1, 10.0];
mjj_sig_alpha2_cat3[1.0, 0.05, 10.0];
mjj_sig_n2_cat3[2.0, 0.1, 5.0];
mjjSig_cat3 = RooDoubleCB(mjj, mjj_sig_m0_cat3, mjj_sig_sigma_cat3, mjj_sig_alpha1_cat3, mjj_sig_n1_cat3, mjj_sig_alpha2_cat3, mjj_sig_n2_cat3);

CMS_hhbbgg_13TeV_mjj_bkg_slope1_cat3[0.9, -1, 10];
CMS_hhbbgg_13TeV_mjj_bkg_slope2_cat3[0.5, -1, 10];
CMS_hhbbgg_13TeV_mjj_bkg_slope3_cat3[0.5, -1, 10];

mjj_hig_slope1_ggh_cat3[0.1, 0, 10];
mjj_hig_slope2_ggh_cat3[0.1, 0, 10];
mjj_hig_slope3_ggh_cat3[0.1, 0, 10];

mjj_hig_slope1_vbf_cat3[0.1, 0, 10];
mjj_hig_slope2_vbf_cat3[0.1, 0, 10];
mjj_hig_slope3_vbf_cat3[0.1, 0, 10];

mjj_hig_m0_tth_cat3[100, 70, 190];
mjj_hig_sigma_tth_cat3[50, 10, 100];
mjj_hig_alpha1_tth_cat3[1.0, 0.01, 10];
mjj_hig_n1_tth_cat3[1, 0.01, 10];
mjj_hig_alpha2_tth_cat3[1.0, 0.01, 10];
mjj_hig_n2_tth_cat3[1, 0.01, 10];
mjjHig_tth_cat3 = RooDoubleCB(mjj, mjj_hig_m0_tth_cat3, mjj_hig_sigma_tth_cat3, mjj_hig_alpha1_tth_cat3, mjj_hig_n1_tth_cat3, mjj_hig_alpha2_tth_cat3, mjj_hig_n2_tth_cat3);

mjj_hig_m0_vh_cat3[100, 70, 190];
mjj_hig_sigma_vh_cat3[50, 10, 100];
mjj_hig_alpha1_vh_cat3[1.0, 0.01, 10];
mjj_hig_n1_vh_cat3[1, 0.01, 10];
mjj_hig_alpha2_vh_cat3[1.0, 0.01, 10];
mjj_hig_n2_vh_cat3[1, 0.01, 10];
mjjHig_vh_cat3 = RooDoubleCB(mjj, mjj_hig_m0_vh_cat3, mjj_hig_sigma_vh_cat3, mjj_hig_alpha1_vh_cat3, mjj_hig_n1_vh_cat3, mjj_hig_alpha2_vh_cat3, mjj_hig_n2_vh_cat3);

mjj_hig_m0_bbh_cat3[100, 10, 180];
mjj_hig_sigma_bbh_cat3[50, 1.0, 100];
mjj_hig_alpha1_bbh_cat3[1.0, 0.01, 10];
mjj_hig_n1_bbh_cat3[1, 0.01, 10];
mjj_hig_alpha2_bbh_cat3[1.0, 0.01, 10];
mjj_hig_n2_bbh_cat3[1, 0.01, 10];
mjjHig_bbh_cat3 = RooDoubleCB(mjj, mjj_hig_m0_bbh_cat3, mjj_hig_sigma_bbh_cat3, mjj_hig_alpha1_bbh_cat3, mjj_hig_n1_bbh_cat3, mjj_hig_alpha2_bbh_cat3, mjj_hig_n2_bbh_cat3);