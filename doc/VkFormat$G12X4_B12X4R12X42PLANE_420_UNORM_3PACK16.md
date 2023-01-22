[`G12X4_B12X4R12X42PLANE_420_UNORM_3PACK16`] specifies an
unsigned normalized *multi-planar format* that has a 12-bit G component
in the top 12 bits of each 16-bit word of plane 0, and a two-component,
32-bit BR plane 1 consisting of a 12-bit B component in the top 12 bits
of the word in bytes 0..1, and a 12-bit R component in the top 12 bits
of the word in bytes 2..3, with the bottom 4 bits of each word unused.
The horizontal and vertical dimensions of the BR plane are halved
relative to the image dimensions, and each R and B value is shared with
the G components for which <span class="katex"><span aria-hidden="true" class="katex-html"><span class="base"><span style="height:1em;vertical-align:-0.25em;" class="strut"></span><span class="minner"><span class="mopen delimcenter" style="top:0em;">⌊</span><span class="mord"><span class="mord mathdefault">i</span><span class="msupsub"><span class="vlist-t vlist-t2"><span class="vlist-r"><span style="height:0.32833099999999993em;" class="vlist"><span style="top:-2.5500000000000003em;margin-left:0em;margin-right:0.05em;"><span class="pstrut" style="height:2.7em;"></span><span class="sizing reset-size6 size3 mtight"><span class="mord mathdefault mtight">G</span></span></span></span><span class="vlist-s">​</span></span><span class="vlist-r"><span class="vlist" style="height:0.15em;"><span></span></span></span></span></span></span><span class="mspace" style="margin-right:0.2222222222222222em;"></span><span class="mbin">×</span><span class="mspace" style="margin-right:0.2222222222222222em;"></span><span class="mord">0</span><span class="mord">.</span><span class="mord">5</span><span class="mclose delimcenter" style="top:0em;">⌋</span></span><span style="margin-right:0.2777777777777778em;" class="mspace"></span><span class="mrel">=</span><span class="mspace" style="margin-right:0.2777777777777778em;"></span></span><span class="base"><span class="strut" style="height:0.80952em;vertical-align:-0.15em;"></span><span class="mord"><span class="mord mathdefault">i</span><span class="msupsub"><span class="vlist-t vlist-t2"><span class="vlist-r"><span style="height:0.32833099999999993em;" class="vlist"><span style="top:-2.5500000000000003em;margin-left:0em;margin-right:0.05em;"><span style="height:2.7em;" class="pstrut"></span><span class="sizing reset-size6 size3 mtight"><span style="margin-right:0.05017em;" class="mord mathdefault mtight">B</span></span></span></span><span class="vlist-s">​</span></span><span class="vlist-r"><span class="vlist" style="height:0.15em;"><span></span></span></span></span></span></span><span style="margin-right:0.2777777777777778em;" class="mspace"></span><span class="mrel">=</span><span class="mspace" style="margin-right:0.2777777777777778em;"></span></span><span class="base"><span class="strut" style="height:0.80952em;vertical-align:-0.15em;"></span><span class="mord"><span class="mord mathdefault">i</span><span class="msupsub"><span class="vlist-t vlist-t2"><span class="vlist-r"><span class="vlist" style="height:0.32833099999999993em;"><span style="top:-2.5500000000000003em;margin-left:0em;margin-right:0.05em;"><span class="pstrut" style="height:2.7em;"></span><span class="sizing reset-size6 size3 mtight"><span class="mord mathdefault mtight" style="margin-right:0.00773em;">R</span></span></span></span><span class="vlist-s">​</span></span><span class="vlist-r"><span class="vlist" style="height:0.15em;"><span></span></span></span></span></span></span></span></span></span> and <span class="katex"><span aria-hidden="true" class="katex-html"><span class="base"><span style="height:1em;vertical-align:-0.25em;" class="strut"></span><span class="minner"><span style="top:0em;" class="mopen delimcenter">⌊</span><span class="mord"><span class="mord mathdefault" style="margin-right:0.05724em;">j</span><span class="msupsub"><span class="vlist-t vlist-t2"><span class="vlist-r"><span style="height:0.32833099999999993em;" class="vlist"><span style="top:-2.5500000000000003em;margin-left:-0.05724em;margin-right:0.05em;"><span class="pstrut" style="height:2.7em;"></span><span class="sizing reset-size6 size3 mtight"><span class="mord mathdefault mtight">G</span></span></span></span><span class="vlist-s">​</span></span><span class="vlist-r"><span class="vlist" style="height:0.15em;"><span></span></span></span></span></span></span><span style="margin-right:0.2222222222222222em;" class="mspace"></span><span class="mbin">×</span><span style="margin-right:0.2222222222222222em;" class="mspace"></span><span class="mord">0</span><span class="mord">.</span><span class="mord">5</span><span style="top:0em;" class="mclose delimcenter">⌋</span></span><span class="mspace" style="margin-right:0.2777777777777778em;"></span><span class="mrel">=</span><span class="mspace" style="margin-right:0.2777777777777778em;"></span></span><span class="base"><span class="strut" style="height:0.85396em;vertical-align:-0.19444em;"></span><span class="mord"><span class="mord mathdefault" style="margin-right:0.05724em;">j</span><span class="msupsub"><span class="vlist-t vlist-t2"><span class="vlist-r"><span style="height:0.32833099999999993em;" class="vlist"><span style="top:-2.5500000000000003em;margin-left:-0.05724em;margin-right:0.05em;"><span style="height:2.7em;" class="pstrut"></span><span class="sizing reset-size6 size3 mtight"><span class="mord mathdefault mtight" style="margin-right:0.05017em;">B</span></span></span></span><span class="vlist-s">​</span></span><span class="vlist-r"><span style="height:0.15em;" class="vlist"><span></span></span></span></span></span></span><span style="margin-right:0.2777777777777778em;" class="mspace"></span><span class="mrel">=</span><span class="mspace" style="margin-right:0.2777777777777778em;"></span></span><span class="base"><span class="strut" style="height:0.85396em;vertical-align:-0.19444em;"></span><span class="mord"><span class="mord mathdefault" style="margin-right:0.05724em;">j</span><span class="msupsub"><span class="vlist-t vlist-t2"><span class="vlist-r"><span style="height:0.32833099999999993em;" class="vlist"><span style="top:-2.5500000000000003em;margin-left:-0.05724em;margin-right:0.05em;"><span style="height:2.7em;" class="pstrut"></span><span class="sizing reset-size6 size3 mtight"><span class="mord mathdefault mtight" style="margin-right:0.00773em;">R</span></span></span></span><span class="vlist-s">​</span></span><span class="vlist-r"><span class="vlist" style="height:0.15em;"><span></span></span></span></span></span></span></span></span></span>.
The location of each plane when this image is in linear layout can be
determined via [`get_image_subresource_layout`], using
`VK_IMAGE_ASPECT_PLANE_0_BIT` for the G plane, and
`VK_IMAGE_ASPECT_PLANE_1_BIT` for the BR plane.
This format only supports images with a width and height that is a
multiple of two.